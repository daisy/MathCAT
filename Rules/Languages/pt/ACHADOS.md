# Achados da tradução do MathCAT para português

Este arquivo registra as descobertas técnicas feitas durante a tradução e a
auditoria da pasta `pt`. Está escrito em linguagem simples, para servir de
consulta daqui a meses — inclusive para quem nunca mexeu no MathCAT.

Índice:

1. [A pausa depende do tamanho das palavras (bug do upstream)](#1)
2. [O que isso muda na prática](#2)
3. [Como abrir o issue no daisy/MathCAT](#3)
4. [Outros achados desta rodada](#4)
5. [Erros que eu mesmo introduzi e depois corrigi](#5)
6. [Rodada de reconciliação dos testes (todos passando)](#6)

---

<a name="1"></a>
## 1. A pausa depende do tamanho das palavras

### O sintoma

Ao rodar `cargo test Languages::pt`, quinze testes de química falhavam sempre
do mesmo jeito: onde o teste esperava uma vírgula, o programa produzia um
ponto e vírgula.

Exemplo com a água (H₂O):

```
teste em inglês (passa):  "cap h, 2 cap o"                 <- vírgula
teste em português:       "maiúsculo h, 2 maiúsculo o"     <- vírgula
o que o programa gerou:   "maiúsculo h, 2, maiúsculo o"    <- vírgula A MAIS
```

O teste em português estava alinhado ao inglês. Era o programa que produzia
uma pausa a mais. E isso acontecia em quase toda a química, sempre na mesma
direção: mais pausa do que deveria.

Um desvio que aparece sempre para o mesmo lado não é acaso. Tem causa.

### A causa

Está no arquivo `src/tts.rs`, na função `compute_auto_pause`:

```rust
let pause = std::cmp::min(3000, ((2 * before_len + after_len)/48) * 128);
```

Traduzindo: o MathCAT decide o tamanho da pausa **contando os bytes** do texto
que vem antes e do que vem depois. Não conta palavras, nem sílabas, nem
estruturas matemáticas. (`str::len()` em Rust conta bytes, não caracteres —
em UTF-8 cada vogal acentuada custa 2. Ver 6.3: trocar por caracteres não
resolve.)

Depois, esse número de milissegundos vira pontuação no texto do teste:

| até 50 ms | até 250 ms | acima disso |
|---|---|---|
| nada | vírgula | ponto e vírgula |

Como a conta multiplica por 128, só existem três resultados possíveis na
prática:

| `(2×antes + depois) ÷ 48` | milissegundos | vira |
|---|---|---|
| 0 | 0 | nada |
| 1 | 128 | vírgula |
| 2 | 256 | ponto e vírgula |

Ou seja: **a fronteira entre vírgula e ponto e vírgula está exatamente em
`2×antes + depois = 96`.** Não existe meio-termo. Passou de 96, a vírgula
vira ponto e vírgula de uma vez.

### Por que isso atinge o português

Porque as palavras em português são mais longas que as em inglês:

| inglês | letras | português | letras | diferença |
|---|---|---|---|---|
| `cap h` | 5 | `maiúsculo h` | 11 | +6 |
| `cap n eigh` | 10 | `maiúsculo n a` | 13 | +3 |
| `open paren` | 10 | `abre parênteses` | 15 | +5 |
| `double bond` | 11 | `ligação dupla` | 13 | +2 |

A palavra "maiúsculo" sozinha tem mais que o dobro de letras de "cap". E em
química ela se repete a cada elemento: "maiúsculo n a, maiúsculo c l,
maiúsculo o..." O texto cresce rápido.

Resultado: uma fórmula que em inglês soma 80 (vírgula) passa de 96 em
português (ponto e vírgula). **A regra de tradução está correta; a conta do
programa é que foi calibrada para o comprimento do inglês.**

### Como verificamos

Comparamos as regras de química do português com as do inglês, linha por
linha. As diretivas de pausa são **idênticas** — nenhuma diferença de `pause:`,
de estrutura ou de condição. Logo, a diferença de pontuação não podia vir
delas, e só podia vir do motor.

> **Ressalva acrescentada depois.** Essa comparação foi feita olhando só as
> pausas, e a conclusão sobre elas continua valendo. Mas as regras de química
> **não eram** idênticas em tudo: a distinção Verbose/Medium estava desabada em
> 14 lugares. Ver 6.1. Lição: "comparei com o inglês" só vale para o que foi
> efetivamente comparado.

---

<a name="2"></a>
## 2. O que isso muda na prática

**Os testes em português devem aceitar o que o programa gera.**

Isso parece "desistir de corrigir", mas não é. A razão importa:

- **Não é** porque o teste estava errado.
- **É** porque a estrutura de pausas em português *legitimamente* difere da
  inglesa. Forçá-la a ser igual à do inglês seria registrar no teste um
  comportamento que o programa não tem — ou seja, falsificar.

Um teste serve para detectar quando algo muda sem querer. Se ele guarda uma
expectativa que o programa nunca vai cumprir, ele deixa de detectar qualquer
coisa e vira ruído.

### Isso quer dizer que a fala está boa?

Não necessariamente, e essa é a parte importante. Mais pausa em português
pode até ser bom (as frases são mais longas, o ouvinte precisa de mais tempo)
ou pode ser ruim (a fala fica arrastada e picotada). **Isso não se decide
lendo código — se decide ouvindo.**

Fica registrado como pergunta para a validação com leitor de tela:

> As pausas na leitura de fórmulas químicas soam naturais, ou a fala parece
> picotada demais?

---

<a name="3"></a>
## 3. Como abrir o issue no daisy/MathCAT

Este é um problema do projeto original, não da tradução. E ele **não afeta só
o português**: atinge qualquer idioma com palavras mais longas que as
inglesas — alemão, espanhol, finlandês, húngaro.

O guia oficial de tradutores já avisava que as pausas foram escolhidas com
base no inglês e que cada tradutor deveria ajustá-las. O que não estava dito
é que **não existe onde ajustar**: o número 48 está fixo no código Rust, não
em nenhum arquivo de regras.

### Sugestões para o issue

1. **Tornar o divisor 48 configurável por idioma**, num arquivo de
   preferências. É a mudança menor e resolve o caso imediato.

2. **Trocar a métrica de bytes para sílabas ou palavras.** Mais correto em
   princípio, porque o que cansa o ouvinte é o tempo de fala, não o número de
   letras. Mas é mudança maior.

3. **Alargar a faixa da vírgula.** Hoje ela vai de 51 a 250 ms e o próximo
   valor possível já é 256 ms — a fronteira é apertada demais para ser
   estável. Qualquer variação pequena de comprimento atravessa o limite.

Vale mencionar no issue que a conta multiplica por 128, então na prática só
existem três resultados, e que isso torna o sistema mais sensível ao idioma
do que provavelmente se pretendia.

---

<a name="4"></a>
## 4. Outros achados desta rodada

### 4.1 As unidades tinham duas grafias e só uma estava coberta

O MathML pode escrever o segundo de duas maneiras: `s` ou `sec`. As duas
estão na lista de unidades do MathCAT.

Nós tínhamos corrigido a ortografia do português (prefixo terminado em vogal
antes de palavra com "s" dobra o s: mili + segundo = **milissegundo**), mas
só para a grafia `s`. Quando o MathML trazia `nsec`, a correção não era
encontrada e o programa montava "nano" + "segundo" = "nanosegundo", com s
simples.

O mesmo valia para o segundo de arco: `zas` virava "zeptosegundo de arco".

Acrescentamos as duas famílias que faltavam. Deixamos de fora, de propósito,
as combinações ambíguas: `Pas` já significa pascal-segundo e `das` já
significa decassegundo.

### 4.2 A probabilidade não dizia "de"

O programa lia `P(x)` como "probabilidade x" em vez de "probabilidade de x".

Causa: no inglês, a definição de `probability` tem **duas formas** de
encaixe na frase, separadas por `||`. Só a segunda produz o "de". A versão
em português tinha só a primeira.

Também estava usando `;` como separador de palavras onde o inglês usa `|`.

### 4.3 Onde procurar quando algo estiver errado

Aprendizado que vale guardar: quando uma fala sai errada, o problema pode
estar em três lugares diferentes, e vale checar nesta ordem:

1. **No arquivo de regras em português** — é o caso mais comum.
2. **No arquivo de teste** — se ele foi editado à mão em algum momento.
3. **No motor, em `src/`** — raro, mas foi o caso das pausas.

A forma de descobrir em qual dos três: **comparar com o inglês.** Se a regra
inglesa e a portuguesa são iguais e o resultado difere, o problema está no
motor. Se as regras diferem, está na tradução.

---

<a name="5"></a>
## 5. Erros que eu mesmo introduzi e depois corrigi

Registrados aqui porque a mesma armadilha pode acontecer de novo.

### 5.1 Uma regra que eu inventei achando que estava portando

Acrescentei uma regra chamada `dot-over` para tratar a notação de Newton
(x com ponto em cima, para derivada). Escrevi no comentário que ela vinha do
inglês.

**Ela não existe no inglês.** O inglês trata esse caso pela regra
`diacriticals`, que simplesmente fala o caractere. Minha regra inventada
ficava antes dela no arquivo e a encobria, produzindo "x com ponto acima"
onde deveria sair "x ponto".

Lição: antes de escrever "portado do inglês" num comentário, conferir se a
regra realmente está lá.

### 5.2 Uniformizei uma distinção que existia de propósito

A seta de reação química (→) é lida como "reage formando" no modo normal e
"forma" no modo conciso. Eu troquei tudo para "reage formando", achando que
era inconsistência. Era distinção deliberada, e precisei reverter.

### 5.3 Mudei um estilo e esqueci o outro

Ao ajustar a função trigonométrica inversa no ClearSpeak para "inversa de",
não olhei o SimpleSpeak, que continuou "seno inverso". Os dois estilos
passaram a divergir sem motivo.

**Corrigido.** As duas regras já diziam "inversa de"; o que tinha ficado para
trás era o *teste*, que ainda esperava "seno inverso". A forma preposta é a
que resolve o gênero: "inversa de seno", "inversa de tangente" e "inversa de
cossecante" funcionam com a mesma palavra, enquanto a posposta exigiria
concordar ("seno inverso", mas "tangente inversa").

### 5.4 Deixei lixo de compilação num commit

Ao tentar compilar com uma versão antiga do Rust, editei arquivos em `src/`
para contornar erros. Esses arquivos quase foram parar num commit da
tradução. Revertidos.

Lição: `git status` antes de todo commit, e olhar se aparece alguma pasta que
não tem nada a ver com o que você estava fazendo.

---

<a name="6"></a>
## 6. Rodada de reconciliação dos testes

Ponto de partida: 33 dos 91 testes de `cargo test Languages::pt` falhavam.
Fim: **92 de 92 passando** (e a suíte inteira do repositório, 7256 testes,
continua verde). O método foi o do cabeçalho dos arquivos de teste: para cada
falha, comparar a regra portuguesa com a inglesa. Se as duas são iguais e só o
resultado difere, o teste é que estava desatualizado; se as regras diferem, a
regra é que estava errada.

### 6.1 Bugs de regra encontrados (7 deles)

**A distinção Verbose/Medium tinha desabado na química.** Em inglês, `H₂` é
"cap h sub 2" em Medium e "cap h subscript 2" em Verbose. A tradução dizia
"subscrito" nos dois. Eram 14 pares de ramos `Verbose`/`Medium` em
`SharedRules/general.yaml` (regras `chemistry-msub`, `chemistry-msup` e
`chemistry-scripts`), todos colapsados. Um deles estava ainda mais trocado:
no ramo `$Prescripts[4]` o Verbose dizia "subscrito" onde o inglês diz
*superscript*.

Isso explicava a contradição registrada nas pendências antigas, em que
`tensor_mmultiscripts` e `mhchem_so4_2mais` pareciam se contradizer no mesmo
nível de verbosidade: são caminhos de código diferentes. O de
`tensor_mmultiscripts` (`SharedRules/default.yaml`) estava correto o tempo
todo; o da química é que estava quebrado.

**A ligação dupla escrita com `::` não era reconhecida.** A regra portuguesa
testava `.='::'`, mas o canonicalizador converte `::` no caractere U+2237
(`∷`) antes de as regras rodarem. O inglês testa `.='∷'`. Resultado: `H₂C::CH₂`
saía "maiúsculo c, **como** maiúsculo c" — o caractere sendo lido pela regra
genérica do unicode — em vez de "ligação dupla".

**A barra vertical perdia três casos.** A regra de `|` em `unicode.yaml` era
uma simplificação da inglesa e só sabia produzir "barra vertical" fora do
ClearSpeak. Faltavam:

- `P(A|B)`, probabilidade condicional, que deve dizer "dado" nos dois estilos;
- `a|b` entre dois números, que deve dizer "divide";
- o caso do intent literal, que deve dizer "barra vertical" sempre.

Portada a estrutura inglesa inteira, com as quatro palavras traduzidas.

**`<none/>` falava em inglês.** Fora da química, o elemento `<none/>` de
`mmultiscripts` não tem regra e cai no `default-text`, que fala o *nome da
tag*. Em inglês isso dá "none", que passa despercebido; em português saía a
palavra inglesa no meio da fala. Acrescentada a regra `none-default`, que diz
"nenhum".

**Faltava o `Log` maiúsculo.** O inglês distingue `log` de `Log` (valor
principal, de variável complexa). A regra portuguesa só casava `log` e `ln`,
então `Log x` caía na regra genérica de `mi` e saía "Log de x". Acrescentado
o ramo, com `t:` minúsculo porque a tradução ("log do valor principal") ainda
não foi verificada por falante nativo.

**Chave duplicada no YAML.** `PluralForms` tinha `"segundo de arco"` duas
vezes. O leitor de YAML do Rust aceita e fica com a última, mas a ferramenta
de auditoria (`uv run --project PythonScripts audit-translations pt`)
recusava o arquivo inteiro por causa disso — ou seja, a auditoria de
`definitions.yaml` não estava rodando de verdade.

### 6.2 Testes que estavam desatualizados

O resto das falhas eram testes que não refletiam as regras. Três grupos:

1. **Pausas.** O grosso. É o problema da seção 1 deste arquivo: as palavras
   portuguesas são mais longas e atravessam o limiar de 96 do
   `compute_auto_pause`. Testes ajustados ao que o motor gera, como decidido
   na seção 2. Casos: toda a química de pausa, `hyperbolic_trig_names`,
   `no_times_sqrt`, `normal_log`, `normal_ln`, `ignore_period`.

2. **Estrutura herdada do inglês que o teste não tinha copiado.** O artigo em
   "o log de x" (o inglês tem "the log of x" e a regra portuguesa tem o mesmo
   `if $Verbosity!='Terse'`); o "1 meio" em vez de "um meio" (o inglês fala
   "1 half" — o numerador vai como dígito, e o sintetizador lê "um"); o modo
   conciso do `ln` sem "de" ("l n x", como "l n x"); e a vírgula do grego
   (`alfa vírgula, ômega`), que o inglês também tem.

3. **Uma pausa que eu tinha tratado como problema de comprimento e não era.**
   Em `alphabets::greek`, a segunda vírgula existe igualmente no teste
   inglês. Não era efeito do português.

### 6.3 Sobre a hipótese de contar caracteres em vez de bytes

`compute_auto_pause` usa `str::len()`, que em Rust conta **bytes**. Como as
vogais acentuadas do português ocupam 2 bytes em UTF-8, "maiúsculo" custa 10 e
não 9. Parecia que trocar por `chars().count()` corrigiria as pausas de graça,
sem mexer em nada específico do português (o inglês é ASCII puro e não mudaria
em nada).

**Testado, e não resolve.** Com a troca, as falhas em `pt` sobem de 15 para
36: quebra as unidades inteiras, `modified_vars`, `trig_names`. O limiar não
está calibrado nem para bytes nem para caracteres — está calibrado para o
comprimento das palavras inglesas, e mudar a unidade só desloca o problema.
Fica registrado como argumento a mais para o issue da seção 3: a métrica certa
não é nenhuma das duas, é tempo de fala.

### 6.4 Decisão tomada nesta rodada

**Menos unário: "menos", não "negativo".** O inglês distingue `infix=minus` de
`prefix=negative`; o espanhol e o alemão seguem essa distinção, o francês, o
norueguês, o sueco e o polonês não. Os testes portugueses estavam divididos:
`no_parens_negative_number` esperava "menos", `ignore_comma` e os dois
`beta_decay` esperavam "negativo". Decidido manter "menos" para os dois usos,
que é o que a regra já fazia, e alinhar os três testes discordantes.

---

## Pendências conhecidas

- A tradução de `Log` ("log do valor principal") está marcada `t:` e precisa
  de conferência.
- Continuam valendo as decisões que dependem de ouvir: se as pausas da química
  soam naturais ou picotadas, e se o artigo em "o log de x" ajuda ou atrapalha.
- O issue da seção 3 (divisor 48 fixo no Rust) não foi aberto ainda.
- **Nada foi ouvido em leitor de tela até agora. Continua sendo o maior
  risco** — todos os testes passarem não diz nada sobre a fala soar bem.
