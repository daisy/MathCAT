# Terminologia do MathCAT em português (pt-BR)

Registro único das leituras adotadas. Serve para que uma decisão terminológica
seja aplicada em **todos** os caminhos que levam à fala, e não só no arquivo
onde alguém estava mexendo — foi a falta de um registro assim que produziu os
casos ℜ/ℑ, ℋ/ℐ e ∈/∉ (ACHADOS 7.8).

**Regra de uso:** antes de mudar uma leitura, procure-a aqui; ao mudar, atualize
esta tabela, todos os arquivos listados em "onde está" e rode
`python3 PythonScripts/conferir_vocabulario.py`.

Status:
- **DEFINIDA** — aplicada em todos os caminhos e travada por teste.
- **PENDENTE DE ESCUTA** — aplicada de forma única, mas a palavra em si
  precisa de conferência com falante nativo / leitor de tela. Entradas `t:`
  minúsculo no YAML.
- **PENDENTE DE DECISÃO** — não foi mexida de propósito; a escolha depende de
  decisão que não é de consistência.

## Pertinência (∈ ∉ ∊)

Princípio (ACHADOS 7.9): **fora** de um conjunto o sujeito é um termo só e o
verbo finito está certo; **dentro** de um conjunto a expressão modifica "todos
os x" — forma nominal, plural, sem verbo finito. ∈ e ∊ falam sempre igual.

| símbolo | contexto | preferência `ClearSpeak_SetMemberSymbol` | leitura adotada | alternativas rejeitadas | status | onde está |
|---|---|---|---|---|---|---|
| ∈ ∊ | fora | Auto, Belongs | pertence a | é membro de (era o Auto) | DEFINIDA | `unicode.yaml` 0x2208, 0x220a |
| ∈ ∊ | fora | Member | é membro de | — | DEFINIDA | idem |
| ∈ ∊ | fora | Element | é um elemento de | — | DEFINIDA | idem |
| ∈ ∊ | fora | In | está em | — | DEFINIDA | idem |
| ∈ ∊ | dentro | Auto, In | em | — | DEFINIDA | idem |
| ∈ ∊ | dentro | Member | membros de | membro de (singular contra "todos os x") | DEFINIDA | idem |
| ∈ ∊ | dentro | Element | elementos de | elemento de (singular) | DEFINIDA | idem |
| ∈ ∊ | dentro | Belongs | pertencentes a | pertence a (verbo finito gruda em "conjunto") | DEFINIDA | idem |
| ∈ ∊ | fora | SimpleSpeak | pertence a | é um elemento de (era o ∊) | DEFINIDA | idem, ramo `SpeechStyle != ClearSpeak` |
| ∈ ∊ | dentro | SimpleSpeak | pertencentes a | pertence a | DEFINIDA | idem |
| ∉ | fora | Auto, Belongs | não pertence a | — | DEFINIDA | `unicode.yaml` 0x2209 |
| ∉ | fora | Member | não é membro de | — | DEFINIDA | idem |
| ∉ | fora | Element | não é um elemento de | — | DEFINIDA | idem |
| ∉ | fora | In | não está em | não está contido em (colidia com ⊄) | DEFINIDA | idem |
| ∉ | dentro | Auto, In, Belongs | não pertencentes a | "não em" (truncado: "não" não nega preposição nua); "fora de" (troca o conceito: afirma localização em vez de negar pertinência; ambíguo com complementar; perde a simetria com o "em" do ∈) | DEFINIDA | idem |
| ∉ | dentro | Member | não membros de | não é membro de | DEFINIDA | idem |
| ∉ | dentro | Element | não elementos de | não é um elemento de | DEFINIDA | idem |
| ∉ | dentro | SimpleSpeak | não pertencentes a | não pertence a | DEFINIDA | idem |

Custo aceito: no ∉ dentro de um conjunto, `Auto|In` e `Belongs` coincidem.
Uma coincidência entre preferências é menos grave que uma troca de conceito.

Testes: `tests/Languages/pt/ClearSpeak/sets.rs` (todas as linhas acima, mais o
guarda `nao_pertinencia_e_nao_subconjunto_falam_diferente`).

## Continência (⊂ ⊃ ⊆ ⊄)

| símbolo | leitura atual | alternativas | status | onde está |
|---|---|---|---|---|
| ⊂ ⊆ | está contido em | é subconjunto de; a ambiguidade próprio/impróprio é real na literatura brasileira | **PENDENTE DE DECISÃO** — sugestão é virar preferência configurável, não escolha fixa (ACHADOS 8.4) | `unicode.yaml` 0x2282, 0x2286 |
| ⊃ | contém | é superconjunto de | PENDENTE DE DECISÃO | `unicode.yaml` 0x2283 |
| ⊄ | não está contido em | — | PENDENTE DE DECISÃO (vai junto com ⊂) | `unicode.yaml` 0x2284 |

Não mexer sem resolver a decisão. O ∉ com `In` foi ajustado justamente para
**não** colidir com "não está contido em".

## Estilos de alfabeto

Ordem: adjetivo de estilo **posposto** em todos os caminhos ("maiúscula a
fraktur"), decisão ACHADOS 8.1. Um termo por família em todos os arquivos.

| família (en) | leitura adotada | alternativa | status | onde está |
|---|---|---|---|---|
| fraktur | fraktur | gótico | PENDENTE DE ESCUTA (`t:` minúsculo em todas as entradas) | `unicode.yaml` ℜ; `unicode-full.yaml` ℌℑℨℭ, 𝔄-𝔜, 𝔞-𝔷, 𝕬-𝖅, 𝖆-𝖟 e cópias MathType |
| script | caligráfico | cursivo | PENDENTE DE ESCUTA (`t:` minúsculo em todas as entradas) | `unicode.yaml` ℋℛℓ; `unicode-full.yaml` ℐℒ℘ℬℰℱℳ, 𝒜-𝒵, 𝒶-𝓏, 𝓐-𝓩, 𝓪-𝔃 e cópias |
| double-struck | vazado | — (já vinha de `alphabets.rs`) | DEFINIDA | `unicode.yaml` ℂℕℚℝℤ, ⅆⅇⅈⅉ; `unicode-full.yaml` ℍℙℾℿ, 𝔸-ℤ, 𝕒-𝕫, 𝟘-𝟡 e cópias |
| bold | negrito | — | DEFINIDA | `unicode-full.yaml` |
| double-struck italic | vazado itálico | — | DEFINIDA | `unicode.yaml` ⅆⅇⅈⅉ |

## Relações (blocos estruturados do `unicode-full.yaml`)

Princípio: termo matemático, não tradução do nome Unicode.

| símbolo | leitura adotada | era | status | onde está |
|---|---|---|---|---|
| ≇ | não é congruente a | nem aproximadamente nem realmente igual a | DEFINIDA | `unicode-full.yaml` 0x2247 |
| ≚ | é equiangular a | é igual e inclinado a | DEFINIDA | 0x225a |
| ≞ | é medido por | é igual por medida a | DEFINIDA | 0x225e |
| ⋕ | é igual e paralelo a | é paralelo forquilhado a | DEFINIDA | 0x22d5 |
| ∦ ⊈ ⊉ | não é paralela a / não está contido em nem é igual a / não contém nem é igual a | mesmas frases com "é" duplicado na frente | DEFINIDA (cópula única; `audit-ignore` explica) | `unicode.yaml` 0x2226; `unicode-full.yaml` 0x2288, 0x2289 |
| ≗ | é igual por definição a | en: 'is approximately equal to' | PENDENTE DE DECISÃO | 0x2257 |
| ⋋ ⋌ | é junção semidireta à esquerda/direita | en: 'semidirect product' | PENDENTE DE DECISÃO | 0x22cb, 0x22cc |
| ⋉ ⋊ | é produto semidireto à esquerda/direita | falta o "de" final do infixo | PENDENTE DE DECISÃO | 0x22c9, 0x22ca |
| ⊶ ⊷ | é imagem original / é imagem | sem o "de" | PENDENTE DE DECISÃO | 0x22b6, 0x22b7 |
| ⋐ ⋑ ⋒ ⋓ ⋔ | leituras próprias | en: double subset / double intersection / proper intersection | PENDENTE DE DECISÃO | 0x22d0-0x22d4 |
| arpões 0x21bc-0x21c3 | nome Unicode completo ("arpão para a esquerda com farpa para cima") | en encurta ("left harpoon up") | DEFINIDA (deliberado: a farpa distingue os oito; nenhum termo matemático perdido) | `unicode-full.yaml` |

## Pontuação e operadores com contexto

| símbolo | contexto | leitura | status | onde está |
|---|---|---|---|---|
| ! | fora de `:literal:` | fatorial | DEFINIDA | `unicode.yaml` 0x21 |
| ! | dentro de `:literal:` | ponto de exclamação (Terse: exclamação) | DEFINIDA | idem |

## Conjuntos numéricos e vazio

| símbolo | leitura | observação | status |
|---|---|---|---|
| ℂ ℕ ℚ ℝ | números complexos / naturais / racionais / reais | — | DEFINIDA |
| ℤ | inteiros | sem "números"; acompanha a assimetria do en ("the integers") | PENDENTE DE DECISÃO (vai junto para o falante nativo) |
| `{ }` | o conjunto vazio | com artigo | DEFINIDA |
| ∅ | conjunto vazio | sem artigo; acompanha o en ('empty set') | PENDENTE DE DECISÃO (idem) |

## Caracteres do corpus real (rodada 9)

| símbolo | leitura adotada | alternativas | status | onde está |
|---|---|---|---|---|
| 𝐀-𝐙 𝐚-𝐳, 𝑨-𝒛 (negrito, negrito itálico) | "x negrito" (posposto) | — | DEFINIDA | `unicode-full.yaml` bloco "Rodada 9" |
| 𝚨-𝛚, 𝜜-𝝎 (grego negrito) | "alfa negrito" | — | DEFINIDA | idem |
| ⟨ ⟩ ⟪ ⟫ | abre/fecha colchete angular (duplo) | "produto interno de" (exige inferência, não palavra) | PENDENTE DE ESCUTA | idem |
| ¯ sobre variável | "x barra" | "x com barra", "média de x" | PENDENTE DE ESCUTA | idem (U+00AF; o motor canonicaliza U+0304/U+203E para ele) |
| ⟵ ⟶ ⟷ ⟸ ⟹ ⟺ | seta longa...; implica; se e somente se | — | DEFINIDA | idem |
| µ Ω ℧ Å Å soltos | mi, ômega, mho, angstrom | "micro" só dentro de unidade | DEFINIDA | idem |
| ∫ e família (artigo) | "a integral" | "o integral" (era) | DEFINIDA | `SharedRules/general.yaml` bigop-both/under, largeop |
| 1/3 … 1/10 | "um terço" … "um décimo"; plural "terços" | "1 terceiro" (era) | DEFINIDA | `definitions.yaml` NumbersOrdinalFractional* |
| ‖x‖ (intent `magnitude`) | "a norma de x" | "magnitude" (vazava en) | DEFINIDA | `definitions.yaml` IntentMappings |
| ⃗a × ⃗b, ⃗a ⋅ ⃗b | "produto vetorial", "produto escalar" | "cross product" (vazava o nome literal) | DEFINIDA | `definitions.yaml` (nofix) + `SharedRules/default.yaml` |
| ∼ em estatística | "varia com" | "tem distribuição", "segue" | PENDENTE DE DECISÃO | `unicode.yaml` ∼ |
| \mid em P(A\|B) (U+2223) | "dado" em P( · ∣ · ); "divide" fora | "divide" em todo lugar (era) | DEFINIDA | `unicode.yaml` 0x2223, mesma condição de contexto do "\|" |

## Recuo controlado

Caracteres sem regra pt recebem a entrada do inglês, gerada por
`PythonScripts/gerar_recuo_en.py` no fim do `unicode-full.yaml` (entre
marcadores). Estilo já definido vira o termo do pt; o resto fica em inglês com
`t:` minúsculo. Status de cada um: **PENDENTE DE ESCUTA** por definição —
traduzir movendo a entrada para cima do marcador.

## Segundo caminho: `IntentMappings` — prioridade baixa, medida

O `definitions.yaml` do pt define 30 dos 216 intents do en. `element-of`,
`member-of`, `not-member-of`, `subset`, `less-than` e todas as trigonométricas
vazam inglês com fixidade errada ("element of de x vírgula") — **mas só se o
MathML trouxer `intent=`, e o do ACESSÍLIA nunca traz** (ACHADOS 9.1). O que
importa são os intents que o motor infere; os que apareceram no corpus
(`magnitude`, `cross-product`, `dot-product`) estão DEFINIDOS acima. Status do
restante: **PENDENTE DE DECISÃO**, a tratar quando algum aparecer na fala.
