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
7. [O caractere não traduzido NÃO cai no inglês](#7)
8. [Três decisões de terminologia (consultoria de acessibilidade)](#8)

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

<a name="7"></a>
## 7. O caractere não traduzido NÃO cai no inglês

Esta seção corrige uma afirmação errada que estava espalhada por três lugares
do projeto e que orientou uma decisão de escopo.

### 7.1 O que se acreditava

Que um caractere ausente do `unicode-full.yaml` do português seria buscado no
arquivo inglês, e que o pior caso seria ouvir a descrição em inglês. Sob essa
premissa, cobrir só quatro faixas do `unicode-full.yaml` parecia uma escolha
segura: o resto "degradaria para o inglês", que é ruim mas inteligível.

### 7.2 O que acontece de verdade

**Sai o caractere cru.** Nem inglês, nem `'\xhhhh'`, nem silêncio: os bytes
UTF-8 do próprio caractere, intactos. Verificado com `hexdump`:

```
U+1D504  𝔄
    en | [fraktur  cap a]   66 72 61 6b 74 75 72 20 20 63 61 70 20 61
    pt | [𝔄]                f0 9d 94 84

U+00BD  ½
    en | [one half]         6f 6e 65 20 68 61 6c 66
    pt | [½]                c2 bd

U+0410  А (cirílico)
    en | [cap a]            63 61 70 20 61
    pt | [А]                d0 90
```

O que o leitor de tela faz com isso depende do sintetizador: pode soletrar o
nome Unicode em inglês, pode ler algo aleatório, pode não dizer nada. Não é
uma degradação controlada — é entregar o problema para a camada de baixo.

### 7.3 Onde isso é decidido

`src/speech.rs`, função `replace_single_char`, depois de falhar a busca nas
duas tabelas (curta e completa):

```rust
            if replacements.is_none() {
                self.translate_count = 0;     // not in loop
                if rules.translate_single_chars_only || ch.is_ascii() {
                  return Ok(self.escape_string_for_safety(String::from(ch)));
                } else {
                  let ch_as_int = ch as u32;
                  if ('\u{2800}'..='\u{28ff}').contains(&ch) {   // braille -- leave as braille
                      return Ok(self.escape_string_for_safety(String::from(ch)));
                  } else {   // Emulate what NVDA does: generate '\xhhhh' or '\yhhhhhh'
                      let prefix_indicator = if ch_as_int < 1<<16 {'x'} else {'y'};
                      return self.replace_chars( &format!("'\\{prefix_indicator}{:06x}'", ch_as_int), mathml);
                  }
                }
              }
```

`String::from(ch)` devolve o caractere inteiro. O ramo `'\xhhhh'`, que imita o
NVDA, **nunca é alcançado na fala**: `translate_single_chars_only` é `true`
para fala e `false` só para braille, como se vê na construção das regras no
mesmo arquivo:

```rust
RefCell::new( SpeechRules::new(RulesFor::Speech, true) );
RefCell::new( SpeechRules::new(RulesFor::Braille, false) );
```

Como a condição é `translate_single_chars_only || ch.is_ascii()`, na fala o
primeiro ramo sempre vence.

### 7.4 Por que não existe recuo para o inglês

Dois motivos, os dois em `src/prefs.rs`:

- `find_file` procura o arquivo **subindo** de `Rules/Languages/pt` em direção
  a `Rules/`, e para ao chegar lá. Nunca olha de lado, para
  `Rules/Languages/en`.
- O `Some("en")` passado nas chamadas de `find_file` só entra em ação em
  `get_language_dir`, quando o **diretório** do idioma não existe. O diretório
  `pt` existe e tem o seu próprio `unicode-full.yaml`.

Ou seja, o recuo para o inglês é de **diretório**, não de conteúdo. Um
`unicode-full.yaml` presente e incompleto é pior do que um ausente: sua mera
existência impede qualquer busca alternativa.

### 7.5 Consequência para o escopo

A decisão de traduzir só quatro faixas do `unicode-full.yaml` (acentos, setas,
operadores e formas geométricas) foi tomada sob a premissa errada de 7.1. Com
o comportamento real, cada caractere não coberto é uma falha muda, não uma
degradação elegante. **Isso muda a prioridade da cobertura do
`unicode-full.yaml`, que deixa de ser acabamento e passa a ser correção.**

Nesta rodada foram acrescentadas as quatro faixas de maior peso pedagógico —
frações vulgares, sobrescritos e subscritos, numerais romanos e os alfabetos
fraktur, vazado e caligráfico. Ver 7.7.

### 7.6 Os comentários errados que ainda precisam ser corrigidos

A afirmação falsa aparece em três lugares. Um deles é esta própria seção, que
a corrige. Os outros dois **continuam errados no código e precisam ser
consertados**:

1. `Rules/Languages/pt/unicode-full.yaml`, no cabeçalho:

   > "Caracteres fora destas faixas caem no inglês, que é o comportamento de
   > degradação seguro do MathCAT (melhor inglês do que soletrar o codepoint)."

   As duas metades estão erradas: não caem no inglês, e não há degradação
   segura. A menção a "soletrar o codepoint" também confunde: é o
   comportamento do braille, não o da fala.

2. `tests/Languages/pt/alphabets.rs`, nos comentários dos testes marcados
   `#[ignore]`:

   > "depende de unicode-full.yaml, removido de pt de propósito (cai no
   > inglês)"

   Além do mecanismo errado, isso torna esses testes inúteis: as strings que
   eles esperam estão em inglês ("fraktur cap eigh comma fraktur cap y") e
   nunca poderiam passar, porque o inglês nunca é consultado. Quando forem
   reativados, precisam ser reescritos, não só destravados.

### 7.7 O que entrou nesta rodada

125 entradas YAML, cobrindo 651 codepoints, todas traduzidas a partir do
inglês, linha a linha:

| Faixa | Codepoints |
|---|---|
| Frações vulgares (0x00bc-0x00be, 0x2150-0x215f) | 19 |
| Sobrescritos e subscritos (0x2070-0x209f) | 41 |
| Numerais romanos (0x2160-0x217f) | 32 |
| Alfabetos fraktur, vazado e caligráfico | 559 |

A cobertura do português passou de 673 para 1324 codepoints (o inglês tem
5075). Nenhuma colisão com o `unicode.yaml`, conferido antes de aplicar.

Três decisões que valem registro:

- **Ordem das palavras.** O inglês antepõe o estilo ("fraktur cap a"); o
  português pospõe o adjetivo. Por isso o `spell:` vem antes da palavra de
  estilo, e sai "maiúsculo a fraktur". Isso é divergência deliberada do en, e
  segue o que o próprio `alphabets.rs` já registrava ("delta maiúsculo
  vazado").
- **"vazado" para *double-struck*** — não é escolha nova, é a que já estava
  em `alphabets.rs`.
- **Numerais romanos ficam como sequência de letras**, igual ao inglês: o
  sintetizador português lê "I V" como "i vê", que é a leitura corrente.

Onze entradas ficaram com `t:` minúsculo, de propósito, por dependerem de
conferência de falante nativo: as nove ocorrências de "fraktur" (a alternativa
é "gótico") e as duas de "elevado à potência zero" / "elevado à potência i"
(a alternativa é a forma cardinal, "elevado a zero"). A auditoria conta essas
onze em "Untranslated text"; as outras 25 que ela acusa são os `translate()`
dos blocos alfabéticos, falso positivo da mesma natureza dos que já existiam.

---

<a name="8"></a>
## 8. Três decisões de terminologia (consultoria de acessibilidade)

Três escolhas de vocabulário que estavam em aberto foram validadas com um
consultor de acessibilidade matemática e aplicadas nesta rodada. Cada uma foi
aplicada isoladamente, com `cargo test Languages::pt` logo em seguida, para
saber o que cada uma quebrava por si só. **Nenhuma quebrou nada: 92/92 antes,
92/92 depois de cada uma.** A suíte completa também segue verde.

### 8.1 "maiúsculo" passa a ser "maiúscula"

**Decisão.** Anteposto à letra, o adjetivo concorda com o substantivo
"letra", implícito — portanto feminino. É também a forma que o guia oficial
do NVDA em pt-BR usa. Onde o português dizia "maiúsculo P", passa a dizer
"maiúscula P".

**Onde ficava, de verdade.** A palavra aparecia 142 vezes no repositório, mas
como **string de fala** só existia em dois lugares, os dois em
`Rules/Languages/pt/unicode.yaml`: a linha 23 (bloco `A-Z`) e a linha 214
(bloco grego maiúsculo). Todo o resto eram expectativas de teste e
comentários. Vale registrar porque a intuição inicial era o contrário — a
impressão de "centenas de strings" vem dos testes, não das regras.

**A sutileza da concordância.** A justificativa (feminino porque concorda com
"letra") só vale para a posição **anteposta**. Posposto a um substantivo
masculino, o correto continuaria sendo o masculino: "delta maiúsculo". Foi
preciso conferir se a forma posposta existe na saída real. **Não existe:** o
motor sempre antepõe a palavra de caixa e pospõe apenas a palavra de estilo.

```
𝔸  ->  "maiúscula a vazado"      (caixa anteposta, estilo posposto)
𝔄  ->  "maiúscula a fraktur"
Δ   ->  "maiúscula delta"
```

Logo a troca é segura em todas as ocorrências vivas.

**Um teste obsoleto encontrado no caminho.** `alphabets.rs:36`
(`greek_mathtype_private`, marcado `#[ignore]`) era a única ocorrência
**posposta** do repositório: `"delta maiúsculo vazado"`. Um `sed` cego teria
produzido "delta maiúscula vazado", que é agramatical. Conferido: essa
expectativa já estava errada por dois motivos independentes — os caracteres
de área privada do MathType não produzem nada em `pt` hoje (a saída é vazia),
e a ordem posposta contradiz tanto o motor quanto o exemplo que o próprio
comentário de `unicode-full.yaml` dá. A linha foi normalizada para a ordem
anteposta. **Continua `#[ignore]` e continua não verificada** — não confie
nela.

Lição repetida de 6.1: substituição em massa de termo precisa de uma passada
pelas posições sintáticas, não só pelo termo.

### 8.2 Numerais romanos falam o valor, não a grafia

**Decisão.** `Ⅳ` (U+2163) deve falar "quatro", não "i v". Soletrar descreve
como o algarismo é **escrito**; quem ouve precisa do **número**. Aplicado a
toda a faixa 0x2160–0x217F (32 entradas, maiúsculas e minúsculas), em
`unicode-full.yaml`.

```
Ⅳ -> "quatro"      Ⅻ -> "doze"      ⅳ -> "quatro"      Ⅿ -> "mil"
```

Isto é uma **divergência deliberada do inglês**, que ainda soletra ("I V").
A justificativa é que esses codepoints são numerais dedicados: a semântica de
número está no próprio caractere, não depende de contexto nenhum.

**O que NÃO foi feito, e por quê.** O "IV" escrito com letras ASCII é outra
história. Ele não passa pelo `unicode-full.yaml`: é marcado por
`src/canonicalize.rs:734` com `data-roman-numeral`, e quem decide a fala são
as regras `default` de `mn` e de `mi` em `SharedRules/default.yaml`, que
mandam `spell: "text()"`. Mudar isso é mexer em **regra de inferência de
contexto**, e é o mesmo ramo de regra que trata `I` e `V` como identificador
ou variável. Ficou pendente, por decisão explícita, aguardando aval.

Nota útil para quem for retomar: o motor **já calcula o valor** e o guarda em
`data-number` (`<mn data-roman-numeral='true' data-number='48'>XLVIII</mn>`).
A mudança seria trocar `spell: "text()"` por algo que leia `@data-number` —
tecnicamente pequena, mas com raio de alcance grande, porque atinge todo
`mn`/`mi` marcado como romano em qualquer estilo de fala.

### 8.3 Expoente zero e expoente i na forma cardinal

**Decisão.** As duas entradas que usavam a forma "elevado à potência X"
passam à forma cardinal, que é a que o resto do sistema já usa:

```
⁰  (0x2070)   "elevado à potência zero"  ->  "elevado a zero"
ⁱ  (0x2071)   "elevado à potência i"     ->  "elevado a i"
```

Ambas foram promovidas de `t:` para `T:`: estavam marcadas como pendentes de
falante nativo e agora estão decididas.

Por que só essas duas: as vizinhas (⁴ a ⁹, ⁿ) usam a forma **ordinal**
("elevado à quarta potência"), que é natural em português para 4ª, 5ª etc.
Zero e `i` não têm ordinal, e por isso tinham caído na construção pesada
"elevado à potência zero". A forma cardinal resolve sem tocar nas ordinais,
que continuam como estavam.

### 8.4 ⊂ ⊃ ⊆ ficaram de fora — e provavelmente deveriam virar preferência

Não foram alterados nesta rodada, por decisão. Fica registrado **por que** a
pendência é real e não apenas falta de tempo.

A ambiguidade é da literatura, não da tradução. Em boa parte do material
brasileiro `⊂` é lido como "está contido em" no sentido de subconjunto
**qualquer** (permitindo a igualdade), e `⊆` aparece como reforço redundante.
Em outra parte, `⊂` é subconjunto **próprio** (exclui a igualdade) e `⊆` é o
que permite. As duas convenções convivem — a mesma instituição (UTFPR) usa
uma em um material e a outra em outro. Não há uma leitura "correta" a ser
descoberta: há duas, e qual delas vale depende do material que o aluno tem na
frente.

Isso muda a natureza do problema. Enquanto se acreditar que existe uma
resposta certa, a tarefa é pesquisar mais. Reconhecida a ambiguidade, a
tarefa passa a ser **não escolher**: uma escolha fixa no `unicode.yaml` vai
estar errada para metade dos usuários, em silêncio e sem recurso.

**Encaminhamento sugerido:** tratar como **preferência configurável**, na
linha do que o MathCAT já faz com `ClearSpeak_Paren`, `ClearSpeak_Fractions`
etc. — uma preferência de estilo com as duas convenções, e um padrão
declarado. Hoje o comportamento é o da primeira convenção ("está contido
em"), fixo, em `unicode.yaml:432` (`⊂`), `:437` (`⊃`) e `:452` (`⊆`).

**Não altere esses três símbolos sem decidir antes a questão da preferência.**
Trocar a convenção fixa só transfere o erro de um grupo de usuários para o
outro.

### 8.5 Cobertura de teste para 8.2 e 8.3

Quando as três decisões foram aplicadas, os 92/92 verdes valeram como prova
para a 8.1 (havia dezenas de casos de química, `intent` e `shared` exercitando
a palavra de caixa) mas **não valiam nada para a 8.2 e a 8.3**: a suíte `pt`
não tinha um único teste tocando a faixa 0x2160-0x217F nem os codepoints
0x2070/0x2071. O verde só dizia "nada mais regrediu".

Isso foi corrigido: `tests/Languages/pt/unicode_full.rs`, 10 testes. As
expectativas foram capturadas da saída real do motor sob as preferências do
próprio harness — não escritas a partir do que se supunha que sairia.

Os testes foram verificados por mutação, isto é, revertendo a decisão e
conferindo que o teste de fato falha:

| mutação aplicada | o que falhou |
|---|---|
| `Ⅳ` de volta para `"I V"` | os 3 testes de romano Unicode que usam `Ⅳ` |
| `⁰` de volta para `"elevado à potência zero"` | os 2 testes de expoente cardinal |
| caminho ASCII lendo `@data-number` | os 2 guardas ASCII (`XIV` virou `"14"`) |

A terceira linha é a que importa mais. Ela simula exatamente a mudança que
ficou pendente em 8.2 — trocar `spell: "text()"` por `@data-number` nas regras
`default` de `mn`/`mi` — e confirma que, se alguém fizer isso sem decidir,
`romanos_ascii_continuam_soletrados_nao_viram_valor` quebra na hora, com
`XIV -> "14"` na mensagem. Era o cenário de regressão que se queria travar.

Note que `letras_ascii_isoladas_continuam_identificadores` **não** quebra nessa
mutação, e está certo: `I` e `V` sozinhos passam pelo ramo
`string-length(.) = 1`, que é outro caminho. O teste existe para documentar
essa separação.

### 8.6 Um efeito colateral encontrado ao escrever os testes

`<msup><mi>x</mi><mi>⁰</mi></msup>` produz **"x elevado a elevado a zero"** —
"elevado a" duplicado. A causa é que o `msup` já diz "elevado a" e o caractere
⁰ carrega a locução inteira. É entrada malformada (o normal seria `<mn>0</mn>`
no expoente, que dá "x elevado a 0"), e a duplicação **é anterior** à decisão
8.3: antes dela a saída era "x elevado a elevado à potência zero". A decisão
não criou o problema, mas deixou-o mais audível. Não foi fixado em teste, para
não carimbar como correta uma saída que não é.

---

## Pendências conhecidas

- A tradução de `Log` ("log do valor principal") está marcada `t:` e precisa
  de conferência.
- Os comentários errados sobre o recuo para o inglês, em
  `unicode-full.yaml` e em `tests/Languages/pt/alphabets.rs`, ainda não
  foram consertados (ver 7.6). O que a rodada 8 mexeu nesses dois arquivos
  foi só a terminologia de caixa e a ordem das palavras em 8.1 — a afirmação
  errada sobre o recuo continua lá.
- "fraktur" contra "gótico": nove entradas ainda marcadas `t:` esperando
  falante nativo. (A forma ordinal contra a cardinal nos sobrescritos saiu
  desta lista — decidida em 8.3.)
- ⊂ ⊃ ⊆ continuam com a convenção fixa "está contido em". A ambiguidade é
  real na literatura brasileira e a sugestão é virar preferência
  configurável, não escolha fixa — ver 8.4. **Não trocar a convenção sem
  resolver isso antes.**
- O "IV" em letras ASCII continua soletrado. Falar o valor exige mexer na
  regra de inferência de contexto em `SharedRules/default.yaml`, o que ficou
  pendente de aval — ver 8.2. Há guarda de teste travando o comportamento
  atual (8.5): a mudança vai quebrar teste de propósito.
- `<msup>` com o caractere ⁰ no expoente duplica o "elevado a" (ver 8.6).
  Entrada malformada, problema anterior à rodada 8, sem teste fixando.
- O `unicode-full.yaml` ainda cobre 1324 dos 5075 codepoints do inglês.
  Depois de 7.5 isso é lacuna de correção, não de acabamento.
- Continuam valendo as decisões que dependem de ouvir: se as pausas da química
  soam naturais ou picotadas, e se o artigo em "o log de x" ajuda ou atrapalha.
- O issue da seção 3 (divisor 48 fixo no Rust) não foi aberto ainda.
- **Nada foi ouvido em leitor de tela até agora. Continua sendo o maior
  risco** — todos os testes passarem não diz nada sobre a fala soar bem.
