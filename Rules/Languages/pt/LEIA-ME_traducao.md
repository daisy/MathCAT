# Pasta pt — tradução do MathCAT para português

## Onde colocar

Coloque esta pasta em:

    MathCAT_ptBR/Rules/Languages/pt

O nome da pasta precisa ser exatamente `pt` (não `pt-br`, não `pt_BR`).
O MathCAT usa o código do idioma para achar os arquivos, e quando alguém
pede `pt-br` e essa variante não existe, ele cai automaticamente no `pt`.
Usar `pt` como base também serve Portugal.

## O que já está feito

Esta pasta foi copiada do ESPANHOL (não do inglês), porque português e
espanhol têm estrutura gramatical parecida — isso adianta bastante o
trabalho em relação a começar do zero ou de tradução automática.

Em cima disso, 29 símbolos foram substituídos pelo léxico do ACESSÍLIA,
já validado em produção. Eles estão marcados com `T:` maiúsculo e têm um
comentário no fim da linha assim:

    - "χ": [T: "qui"]    # ACESSILIA (es: "ji")

O comentário serve para você conferir depois o que foi mudado e por quê.

As correções mais importantes foram nas letras gregas, onde o espanhol
usa transliteração do inglês:

    θ   es: "theta"    ->  pt: "teta"
    φ   es: "phi"      ->  pt: "fi"
    χ   es: "ji"       ->  pt: "qui"
    ρ   es: "rho"      ->  pt: "rô"
    κ   es: "kappa"    ->  pt: "capa"
    μ   es: "my"       ->  pt: "mi"
    ν   es: "ny"       ->  pt: "ni"
    ξ   es: "xi"       ->  pt: "csi"
    υ   es: "ípsilon"  ->  pt: "úpsilon"
    ω   es: "omega"    ->  pt: "ômega"
    γ   es: "gamma"    ->  pt: "gama"

## O que falta fazer

### 1. unicode.yaml — 59 entradas ainda em espanhol

São as que continuam com `t:` minúsculo. Procure por `t:` no arquivo e
traduza uma por uma. Quando conferir e estiver certo, troque o `t:` por
`T:` maiúsculo.

REGRA DE OURO: só troque para maiúsculo depois de OLHAR aquela entrada.
Essa marcação é o que registra que um humano conferiu. Foi exatamente
isso que faltou no Access8Math e por isso ele ficou com "chi" e
"muito menos que".

### 2. definitions.yaml — números

Números cardinais e ordinais, usados para falar coisas como
"três quintos". Estão em espanhol; traduzir.

### 3. SimpleSpeak_Rules.yaml e SharedRules/

As palavras de estrutura: fração, potência, raiz, "de", "sobre",
"fim da fração". Aqui valem as decisões que o ACESSÍLIA já tomou:

    "ao quadrado"  (não "quadrada")
    "ao cubo"      (não "cubo")
    "raiz quadrada de" ... "fim da raiz"
    "fração com numerador ... e denominador ... fim da fração"
    "é menor ou igual a"  (com o verbo)
    "pertence a"          (com a preposição)

### 4. navigate.yaml

Comandos de navegação. É longo (1.773 linhas), mas muitas palavras se
repetem — dá para fazer boa parte com localizar e substituir.

## O que foi deixado de fora de propósito

- ClearSpeak_Rules.yaml  -> renomeado para .untranslated
  O guia oficial recomenda escolher UM estilo de fala para começar.
  Escolhemos SimpleSpeak. Se um dia quiser traduzir o ClearSpeak,
  é só tirar o .untranslated do nome.

- unicode-full.yaml  -> removido desta pasta
  São 3.401 linhas de símbolos de matemática muito avançada e rara.
  O guia diz explicitamente para deixar por último. Quando quiser,
  copie de Rules/Languages/es/unicode-full.yaml

- overview.yaml  -> removido desta pasta
  Menos crítico, não afeta a leitura das expressões diretamente.
  Copie do espanhol quando for tratar dele.

## Como testar sem git

Copie a pasta `pt` para dentro da instalação do NVDA, em:

    NVDA\include\nvda-mathcat\assets\Rules\Languages

Abra o NVDA, vá em Preferências > Configurações do MathCAT, e o idioma
novo deve aparecer na lista de idiomas. Selecione e teste.

Boa página com expressões prontas para ouvir:
    https://daisy.github.io/MathCAT/test-exprs.html

Faça uma mudança de cada vez, ouça de novo, anote. O guia insiste nisso:
uma tradução pode parecer perfeita no YAML e soar confusa quando falada.

## Contato útil

O guia de tradutores foi escrito por quem fez as traduções para norueguês
e sueco, e eles se oferecem para ajudar quem está começando:

    mathcat-wg@daisylists.org

---

# ATUALIZAÇÃO — unicode.yaml e unicode-full.yaml

## unicode.yaml: TRADUZIDO (198 strings)

Traduzido a partir do SENTIDO EM INGLÊS (os comentários do próprio
arquivo trazem o original), não da tradução espanhola — que tinha erros.

### Erros do espanhol que NÃO foram copiados

O arquivo espanhol tem um trecho onde as traduções saíram trocadas entre
símbolos. Corrigidos aqui:

    caractere      espanhol dizia          correto (inglês)     agora em pt
    "              barra inversa           quotation mark       aspas
    \              se abren corchetes      back slash           barra invertida
    ℂℕℚℝℤ          prima inversa           double-struck        vazado
    ℋℛℓ            grados Fahrenheit       script               cursivo
    ℜ              r mayúscula partida     fraktur              gótico
    ⅆⅇⅈⅉ           angstrom resaltado      double-struck ital.  vazado itálico

Também corrigido: "grados Kelvin" -> "kelvin" (kelvin não leva "grau").

### Letras gregas — léxico ACESSÍLIA

O espanhol mantinha transliteração inglesa. Substituído:

    θ  theta   -> teta        φ  phi     -> fi
    χ  ji      -> qui         ρ  rho     -> rô
    κ  kappa   -> capa        μ  my      -> mi
    ν  ny      -> ni          ξ  xi      -> csi
    υ  ípsilon -> úpsilon     ω  omega   -> ômega
    γ  gamma   -> gama        ο  ómicron -> ômicron

### Concordância de gênero

Um mesmo "el" em espanhol precisava de gêneros diferentes em português.
Resolvido caractere a caractere:

    ∆  ->  "o" incremento        (masculino)
    √  ->  "a" raiz quadrada     (feminino)

Por isso a tradução foi feita por CARACTERE, e não por substituição
global de texto: uma substituição global teria errado um dos dois.

## unicode-full.yaml: REMOVIDO DE PROPÓSITO

Este arquivo NÃO está na pasta, e isso é intencional.

São 3.401 linhas com 2.311 textos distintos, quase todos de símbolos
raríssimos (setas com farpas, arpões, variantes de círculo). O guia
oficial de tradutores diz explicitamente para deixar este arquivo por
último.

Por que remover em vez de deixar em espanhol:

O MathCAT procura o arquivo na pasta do idioma e, se não achar, cai no
INGLÊS (verificado no código-fonte, src/prefs.rs, função find_file, que
recebe Some("en") como idioma padrão).

Ou seja:

  - deixando o arquivo em espanhol  -> símbolo raro é lido em espanhol,
    que é parecido demais com português e o erro passa despercebido;
  - removendo o arquivo             -> símbolo raro é lido em inglês,
    o que é obviamente estrangeiro e fácil de perceber e reportar.

A segunda opção é mais honesta e mais segura. Quando quiser traduzir,
copie de Rules/Languages/es/unicode-full.yaml (ou de en/) e traduza aos
poucos, mantendo `t:` minúsculo até conferir cada entrada.

ATENÇÃO: não traduza esse arquivo com tradutor automático de uma vez.
É exatamente assim que o Access8Math acabou com "chi", "cosine" e
"muito menos que" no dicionário em português dele.

---

# ATUALIZAÇÃO — SimpleSpeak_Rules.yaml

Traduzido. Mas o arquivo em espanhol tinha CINCO defeitos reais que
não foram copiados. Todos estão marcados com "# CORREÇÃO:" no arquivo.

## 1. "cúbica" em vez de "ao cubo"  (2 ocorrências)

O espanhol dizia:

    se expoente = 2  ->  "al cuadrado"     (locução adverbial)
    se expoente = 3  ->  "cúbica"          (adjetivo feminino solto)

Isso é incoerente dentro da própria regra, e o inglês é "squared"/"cubed".
Em espanhol "x cúbica" não faz sentido; concorda com um substantivo
feminino que não está ali.

    pt-BR:  "ao quadrado" / "ao cubo"

Confere com o léxico do ACESSÍLIA, que já produz "x ao quadrado" e
"x ao cubo".

## 2. Sufixo inglês "-th" deixado no arquivo  (2 ocorrências)

Nas regras `root` e `simple-var`, o espanhol manteve intacto:

    pronounce: [text: "-th", ipa: "θ", sapi5: "th", eloquence: "T"]

Isso existe porque em inglês se diz "n-th root" e "x to the n-th".
Em espanhol e em português esse sufixo não existe — o leitor de tela
emitia um som estrangeiro no meio da frase.

    Removido. Em português:
      raiz   ->  "raiz de índice n"
      potência -> "x elevado a n"

## 3. "fin de la exponente" — erro de gênero

"exponente" é masculino. Mesmo em espanhol o correto seria
"fin del exponente".

    pt-BR:  "fim do expoente"

## 4. "conjunto completo" em vez de "conjunto de todos os"

O inglês é "set of all", usado na notação { x : condição } — o conjunto
de TODOS os x tais que. O espanhol traduziu como "conjunto completo",
que significa outra coisa (conjunto completo é conceito distinto em
matemática).

    pt-BR:  "conjunto de todos os"

## 5. Concordância de gênero nos artigos

O espanhol usa "la"/"el" e o português precisa de gêneros diferentes
nos mesmos pontos:

    raiz     ->  "a raiz quadrada"   (feminino)
    conjunto ->  "o conjunto vazio"  (masculino)
    potência ->  "a ... potência de" (feminino)

## Decisões de vocabulário alinhadas ao ACESSÍLIA

    fração simples      ->  "a sobre b"          (não "a partido por b")
    fração completa     ->  "fração ... sobre ... fim da fração"
    raiz                ->  "raiz quadrada de ... fim da raiz"
    multiplicação       ->  "vezes"  /  "multiplicado por" (modo verboso)
    expoente inteiro    ->  "x elevado a quatro"
    unidade             ->  "cinco metros por segundo"

## Observação sobre expoentes ordinais

Na regra `simple-integer` o espanhol usava ToOrdinal para gerar
"a la cuarta" (subentendendo "potencia"). Em português do Brasil a forma
corrente é "elevado a quatro", não "à quarta", então a regra foi
simplificada para ler o número diretamente.

Se preferir a forma "elevado à quarta potência", basta trocar
`x: "*[2]"` por `x: "ToOrdinal(*[2])"` e acrescentar `T: "potência"`.
Vale testar as duas com usuários antes de decidir.

---

# ATUALIZAÇÃO — SharedRules/ (5 arquivos, ~1.700 linhas)

Todos traduzidos. 195 strings. Zero resíduo em minúsculo.

## Achado mais grave: calculus.yaml NUNCA foi traduzido

O arquivo estava 100% em INGLÊS no pacote espanhol — nenhuma entrada em
maiúscula, todas as 6 em minúsculo:

    "divergence" / "div"   ->  "divergente" / "div"
    "of"                   ->  "de"
    "curl of"              ->  "rotacional de"
    "gradient of"          ->  "gradiente de"
    "del"                  ->  "nabla"

Ou seja: quem usasse o MathCAT em espanhol para ler cálculo vetorial
ouvia "curl of" e "gradient of" no meio da frase em espanhol.

## Erros de tradução do espanhol corrigidos

### 1. "regla" em vez de "norma"  (linear-algebra.yaml)

O inglês é "norm" — a norma de um vetor. O espanhol traduziu como
"regla", que significa régua / regra. Nada a ver.

    pt-BR:  "norma"

Este mesmo erro também aparecia no definitions.yaml, na entrada
IntentMappings "norm", e já foi corrigido lá.

### 2. "con acceso" em vez de "com o elemento"  (general.yaml, 2x)

O inglês é "with entry" — a entrada / o elemento de uma matriz.
O espanhol traduziu "entry" como "acceso" (acesso), confundindo com
o sentido de "entrada de porta".

    pt-BR:  "com o elemento"

Aparece nas regras 1x1-matrix e 1x1-determinant.

### 3. "natural log" deixado em inglês  (general.yaml)

Passou sem tradução no arquivo espanhol.

    pt-BR:  "logaritmo natural"

### 4. "and alternating scripts" e "end scripts" em inglês (default.yaml)

Também passaram sem tradução.

    pt-BR:  "e índices alternados"  /  "fim dos índices"

### 5. Expressões IfThenElse com texto inglês dentro (default.yaml, 4x)

Estas linhas têm texto FALADO dentro de uma expressão, e por isso
escaparam da tradução:

    "IfThenElse($Verbosity='Verbose', 'pre subscript', 'pre sub')"
    "IfThenElse($Verbosity='Verbose', 'subscript', 'sub')"
    ... e mais duas

Traduzidas para 'pré-subscrito'/'pré-sub', 'subscrito'/'sub', etc.

### 6. "segmento de línea" -> "segmento de reta"  (geometry.yaml)

Em geometria, no Brasil, o termo é "segmento de reta", não
"segmento de linha".

### 7. "da forma" -> "reage formando"  (general.yaml)

O inglês é "reacts to form", usado em equações químicas
(hidrogênio e oxigênio "reagem formando" água). A tradução espanhola
perdeu o verbo.

## Coerência garantida entre arquivos

Estes termos aparecem em mais de um arquivo e foram unificados:

    fração simples   ->  "sobre"           (não "partido por")
    fim de fração    ->  "fim da fração"
    subscrito        ->  "subscrito"       (não "subíndice")
    sobrescrito      ->  "sobrescrito"     (não "superíndice")
    linha de tabela  ->  "linha"           (es usava "fila")
    célula           ->  "célula"          (es usava "celda")
    potência         ->  "elevado à ... potência"

## Química — nota

O bloco de estados físicos e ligações foi adaptado ao uso brasileiro:

    "enlace único"     ->  "ligação simples"
    "doble enlace"     ->  "ligação dupla"
    "triple enlace"    ->  "ligação tripla"
    "cuádruple enlace" ->  "ligação quádrupla"
    "acuoso"           ->  "aquoso"
    "gaseoso"          ->  "gasoso"

## Situação geral da pasta

    TRADUZIDO:  unicode.yaml, definitions.yaml, SimpleSpeak_Rules.yaml,
                navigate.yaml, overview.yaml, SharedRules/ (5 arquivos)

    FORA:       ClearSpeak_Rules.yaml (renomeado .untranslated)
                unicode-full.yaml (removido; cai no inglês)

Falta agora: ouvir tudo no NVDA e ajustar pausas, revisar com um
matemático e testar com usuários cegos.

---

# REVISÃO FINAL — correções desta rodada

## 1. Bug de codepoints sósias no definitions.yaml (CORRIGIDO)

Os pares K/K, Ω/Ω e Å/Å usam codepoints DIFERENTES que parecem
idênticos (letra K vs U+212A SINAL KELVIN; ômega grego vs U+2126 SINAL
OHM; Å letra vs U+212B SINAL ANGSTRÖM). Na reescrita do arquivo os dois
de cada par tinham colapsado no mesmo caractere — o sinal Unicode ficou
sem mapeamento. Restaurados por código:

    0x4b  e 0x212a -> kelvin
    0x3a9 e 0x2126 -> ohm
    0xc5  e 0x212b -> angström

Lição: nunca digitar pares de sósias Unicode à mão; sempre gerar por
código com o codepoint explícito.

## 2. Símbolos de alto valor acrescentados ao unicode.yaml

Não existiam no espanhol (caíam no unicode-full em inglês):

    ≪  é muito menor que      (o Access8Math diz "muito menos que" — errado)
    ≫  é muito maior que
    ⊥  é perpendicular a      (o Access8Math diz "tacha para cima")
    ̌  com cáron               ̈  com trema
    ⃛  com três pontos acima   ⃜  com quatro pontos acima

Os quatro últimos servem às variáveis modificadas (x com trema =
segunda derivada na notação de Newton).

## 3. Regra dos intervalos corrigida (SharedRules/general.yaml)

O original (inclusive em espanhol) falava o NOME INGLÊS da tag
("open interval") via translate(name(.)). Agora fala "intervalo
aberto/fechado/aberto fechado/fechado aberto" conforme o caso.

## 4. Modo Terse da trigonometria — lixo de tradução automática NÃO herdado

No espanhol, as abreviações fonéticas inglesas foram traduzidas como
palavras: sec Terse era "seek" -> virou "hallar" (o verbo achar!);
sinh era "sinch" -> leram "since" -> virou "puesto que". Em pt:

    sen, cos, tan, sec (curtas pronunciáveis)
    formas hiperbólicas Terse = forma completa por extenso

## 5. Ordem "maiúsculo" + letra

A regra fala "maiúsculo" ANTES da letra ("maiúsculo p"). Os testes
foram unificados nessa ordem. Se o teste com usuários preferir
"p maiúsculo", a mudança é num único lugar do unicode.yaml
(regras A-Z e Α-Ω) — e os testes se reconciliam pelo cargo.

## 6. O que segue pendente (honesto)

- Ouvir tudo no NVDA e calibrar pausas (nunca foi feito para pt).
- Revisão por matemático nativo (orientador).
- Teste com usuários cegos — inclusive as escolhas em aberto:
  "elevado a quatro" vs "elevado à quarta potência";
  "maiúsculo p" vs "p maiúsculo";
  → como "tende a" (contexto de limite) vs "seta para a direita".
- unicode-full.yaml (2.311 textos) — deixado por último de propósito.
- navigate.yaml usa termos de comando que merecem validação com quem
  usa NVDA de verdade em português.
