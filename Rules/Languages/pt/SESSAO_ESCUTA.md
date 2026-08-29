# Sessão de escuta — MathCAT pt-BR

Pacote gerado por `PythonScripts/gerar_pacote_escuta.py`. Não edite à mão:
regenere depois de qualquer mudança nas regras, para a fala aqui ser a fala real.

30 expressões, 6 áreas, MathML produzido pelo `latex2mathml` (o caminho do ACESSÍLIA).
Para cada uma: LaTeX, MathML, e a fala do motor em ClearSpeak e SimpleSpeak,
nos três níveis de verbosidade (Terse / Medium / Verbose).


## Como conduzir

Uma expressão por vez. Leia a fala **em voz alta, com o leitor de tela**, não
com a sua voz — o que está em teste é a pronúncia do sintetizador sobre estas
palavras, tanto quanto as palavras. Comece pelo nível **Medium** (é o padrão
do MathCAT); use Verbose e Terse só nas que gerarem dúvida.

Para cada expressão, na ordem, sem mostrar a fórmula antes:

1. **Reescreva o que ouviu.** (A pergunta mais valiosa. Entregue papel ou
   editor e peça a expressão como o ouvinte a entendeu. Só depois mostre o
   LaTeX. Onde a reescrita divergir, anote *o que* divergiu — expoente,
   limite da fração, sinal, agrupamento.)
2. **Onde começa e onde termina a fração?** (Nas expressões com fração
   aninhada — Bhaskara, fração de fração, variância — o ouvinte consegue dizer
   o que está no numerador e o que está no denominador?)
3. **Qual era o expoente?** (Nas que têm potência: "ao quadrado", "elevado a
   x", "elevado a menos 1". Pergunte também o *que* está elevado: "e elevado a
   x" ou "x e, elevado a x"?)
4. **Soou picotada?** (Muitas pausas — ver ACHADOS 1 e 2: as pausas são
   calculadas em bytes e o português é mais longo que o inglês. Na química é
   onde mais aparece.)
5. **Alguma palavra soou estrangeira ou estranha?** (Anote a palavra exata.
   É aqui que "fraktur", "vazado", "colchete angular", "mho" e "angstrom"
   vão aparecer, se aparecerem.)
6. **Faltou alguma coisa?** (Algo que estava na fórmula e não foi dito, ou
   foi dito e não estava.)

Registre as respostas na tabela ao fim deste arquivo.

## Decisões que dependem desta sessão

Estas estão em `TERMINOLOGIA_PT_BR.md` com status **PENDENTE DE ESCUTA**. A
sessão é o que as fecha.

| decisão | como está hoje | alternativa | onde ouvir |
|---|---|---|---|
| **fraktur × gótico** | "fraktur" (ex.: "maiúscula r fraktur") | "gótico" | não há fraktur nas 30 expressões de propósito — é raro em graduação. Pergunte diretamente: "se eu disser *maiúscula a fraktur*, você entende que letra é?" |
| **vazado × traço duplo** | "vazado" (ℝ é falado "números reais"; 𝔸 solto é "maiúscula a vazado") | "traço duplo", "duplo traço" | Sistema (ℝⁿ), Continência (ℕ), Pertinência (ℤ) — mas note que os conjuntos numéricos nomeados **não** usam a palavra; só letras avulsas usam |
| **maiúscula anteposta** | "maiúscula a" (antes da letra) | "a maiúscula" (depois) | qualquer expressão com A, B, N, P: Matriz, União e interseção, Probabilidade condicional |
| **pausas da química** | vírgula a cada elemento | menos pausas (exige mudança no motor, ver ACHADOS 3) | as cinco de Química |
| **"x barra"** para x̄ | "x barra" | "x com barra", "média de x" | Média amostral, Normal |
| **"colchete angular"** para ⟨u, v⟩ | "abre colchete angular, u vírgula, v, fecha colchete angular" | "produto interno de u e v" (exige inferência de intent, não só palavra) | Produto interno e norma |
| **"varia com"** para ∼ | "varia com" | "tem distribuição", "segue" (em estatística ∼ é distribuição, não proporcionalidade) | Normal |
| **"seta para a direita"** em f: ℝ → ℝ | "números reais seta para a direita números reais" | "em", "para" | não está nas 30; pergunte se surgir |
| **∼ / ≃ / ≈** | "varia com" / "é assintoticamente igual a" / "é aproximadamente igual a" | — | Normal |



## Aritmética

### 1. Fração mista e decimal

`3\frac{1}{2} + 0{,}75 = 4{,}25`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mn>3</mn><mfrac><mrow><mn>1</mn></mrow><mrow><mn>2</mn></mrow></mfrac><mo>&#x0002B;</mo><mn>0</mn><mrow><mo>&#x0002C;</mo></mrow><mn>75</mn><mo>&#x0003D;</mo><mn>4</mn><mrow><mo>&#x0002C;</mo></mrow><mn>25</mn></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | 3 e 1 meio mais 0,75; igual a 4,25 |
| ClearSpeak | Medium | 3 e 1 meio mais 0,75; é igual a 4,25 |
| ClearSpeak | Verbose | 3 e 1 meio mais 0,75; é igual a 4,25 |
| SimpleSpeak | Terse | 3 e 1 meio mais 0,75; igual a 4,25 |
| SimpleSpeak | Medium | 3 e 1 meio mais 0,75; é igual a 4,25 |
| SimpleSpeak | Verbose | 3 e 1 meio mais 0,75; é igual a 4,25 |

### 2. Bhaskara

`x = \frac{-b \pm \sqrt{b^{2} - 4ac}}{2a}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>x</mi><mo>&#x0003D;</mo><mfrac><mrow><mo>&#x02212;</mo><mi>b</mi><mi>&#x000B1;</mi><msqrt><mrow><msup><mi>b</mi><mrow><mn>2</mn></mrow></msup><mo>&#x02212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | x igual a; a fração com numerador; menos b mais ou menos; raiz quadrada, b ao quadrado menos 4 a c; e denominador 2 a |
| ClearSpeak | Medium | x é igual a; a fração com numerador; menos b mais ou menos; a raiz quadrada de b ao quadrado menos 4 a c; e denominador 2 a |
| ClearSpeak | Verbose | x é igual a; a fração com numerador; menos b mais ou menos; a raiz quadrada de b ao quadrado menos 4 a c, fim da raiz; e denominador 2 a; fim da fração |
| SimpleSpeak | Terse | x igual a; fração, menos b mais ou menos; raiz quadrada, b ao quadrado menos 4 a c fim da raiz; sobre, 2 a, fim da fração |
| SimpleSpeak | Medium | x é igual a; fração, menos b mais ou menos; a raiz quadrada de b ao quadrado menos 4 a c fim da raiz; sobre, 2 a, fim da fração |
| SimpleSpeak | Verbose | x é igual a; fração, menos b mais ou menos; a raiz quadrada de b elevado ao quadrado menos 4 a c fim da raiz; sobre, 2 a, fim da fração |

### 3. Potência e raiz

`\sqrt[3]{27} = 3^{1} \quad \text{e} \quad 2^{10} = 1024`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mroot><mrow><mn>27</mn></mrow><mn>3</mn></mroot><mo>&#x0003D;</mo><msup><mn>3</mn><mrow><mn>1</mn></mrow></msup><mspace width="1em" /><mtext>e</mtext><mspace width="1em" /><msup><mn>2</mn><mrow><mn>10</mn></mrow></msup><mo>&#x0003D;</mo><mn>1024</mn></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | raiz cúbica 27, igual a, 3 elevado a 1 e 2 elevado a 10; igual a 1024 |
| ClearSpeak | Medium | a raiz cúbica de 27; é igual a, 3 elevado a 1 e 2 elevado a 10; é igual a 1024 |
| ClearSpeak | Verbose | a raiz cúbica de 27, fim da raiz; é igual a, 3 elevado a 1 e 2 elevado a 10; é igual a 1024 |
| SimpleSpeak | Terse | raiz cúbica 27, igual a, 3 elevado a 1 e 2 elevado a 10; igual a 1024 |
| SimpleSpeak | Medium | a raiz cúbica de 27; é igual a, 3 elevado a 1 e 2 elevado a 10; é igual a 1024 |
| SimpleSpeak | Verbose | a raiz cúbica de 27; é igual a, 3 elevado a 1 e 2 elevado a 10; é igual a 1024 |

### 4. Porcentagem

`15\% \cdot 200 = 30`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mn>15</mn><mi>&#x00025;</mi><mo>&#x000B7;</mo><mn>200</mn><mo>&#x0003D;</mo><mn>30</mn></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | 15 por cento vezes 200, igual a 30 |
| ClearSpeak | Medium | 15 por cento vezes 200; é igual a 30 |
| ClearSpeak | Verbose | 15 por cento, multiplicado por 200; é igual a 30 |
| SimpleSpeak | Terse | 15 por cento vezes 200, igual a 30 |
| SimpleSpeak | Medium | 15 por cento vezes 200; é igual a 30 |
| SimpleSpeak | Verbose | 15 por cento, multiplicado por 200; é igual a 30 |

### 5. Fração de fração

`\frac{\frac{1}{2}}{\frac{3}{4}} = \frac{2}{3}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mfrac><mrow><mfrac><mrow><mn>1</mn></mrow><mrow><mn>2</mn></mrow></mfrac></mrow><mrow><mfrac><mrow><mn>3</mn></mrow><mrow><mn>4</mn></mrow></mfrac></mrow></mfrac><mo>&#x0003D;</mo><mfrac><mrow><mn>2</mn></mrow><mrow><mn>3</mn></mrow></mfrac></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | 1 meio sobre 3 quartos, igual a 2 terços |
| ClearSpeak | Medium | 1 meio sobre 3 quartos, é igual a 2 terços |
| ClearSpeak | Verbose | 1 meio sobre 3 quartos, é igual a 2 terços |
| SimpleSpeak | Terse | fração, 1 meio, sobre, 3 quartos, fim da fração; igual a 2 terços |
| SimpleSpeak | Medium | fração, 1 meio, sobre, 3 quartos, fim da fração; é igual a 2 terços |
| SimpleSpeak | Verbose | fração, 1 meio, sobre, 3 quartos, fim da fração; é igual a 2 terços |


## Conjuntos e lógica

### 6. Pertinência e compreensão

`\{ x \in \mathbb{Z} : x > 5 \}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo stretchy="false">&#x0007B;</mo><mi>x</mi><mo>&#x02208;</mo><mi>&#x02124;</mi><mi>:</mi><mi>x</mi><mo>&#x0003E;</mo><mn>5</mn><mo stretchy="false">&#x0007D;</mo></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | conjunto de todos os x em inteiros tal que x maior que 5 |
| ClearSpeak | Medium | o conjunto de todos os x em inteiros tal que x é maior que 5 |
| ClearSpeak | Verbose | o conjunto de todos os x em inteiros tal que x é maior que 5 |
| SimpleSpeak | Terse | conjunto de todos os x pertencentes a inteiros tal que x maior que 5 |
| SimpleSpeak | Medium | o conjunto de todos os x pertencentes a inteiros tal que x é maior que 5 |
| SimpleSpeak | Verbose | o conjunto de todos os x pertencentes a inteiros tal que x é maior que 5 |

### 7. União e interseção

`(A \cup B) \cap C = (A \cap C) \cup (B \cap C)`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo stretchy="false">&#x00028;</mo><mi>A</mi><mo>&#x0222A;</mo><mi>B</mi><mo stretchy="false">&#x00029;</mo><mo>&#x02229;</mo><mi>C</mi><mo>&#x0003D;</mo><mo stretchy="false">&#x00028;</mo><mi>A</mi><mo>&#x02229;</mo><mi>C</mi><mo stretchy="false">&#x00029;</mo><mo>&#x0222A;</mo><mo stretchy="false">&#x00028;</mo><mi>B</mi><mo>&#x02229;</mo><mi>C</mi><mo stretchy="false">&#x00029;</mo></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | abre parênteses, maiúscula a união maiúscula b; fecha parênteses; interseção maiúscula c; igual a; abre parênteses, maiúscula a interseção maiúscula c; fecha parênteses; união; abre parênteses, maiúscula b interseção maiúscula c; fecha parênteses |
| ClearSpeak | Medium | abre parênteses, maiúscula a união maiúscula b; fecha parênteses; interseção maiúscula c; é igual a; abre parênteses, maiúscula a interseção maiúscula c; fecha parênteses; união; abre parênteses, maiúscula b interseção maiúscula c; fecha parênteses |
| ClearSpeak | Verbose | abre parênteses, maiúscula a união maiúscula b; fecha parênteses; interseção maiúscula c; é igual a; abre parênteses, maiúscula a interseção maiúscula c; fecha parênteses; união; abre parênteses, maiúscula b interseção maiúscula c; fecha parênteses |
| SimpleSpeak | Terse | abre parênteses, maiúscula a união maiúscula b; fecha parênteses; interseção maiúscula c; igual a; abre parênteses, maiúscula a interseção maiúscula c; fecha parênteses; união; abre parênteses, maiúscula b interseção maiúscula c; fecha parênteses |
| SimpleSpeak | Medium | abre parênteses, maiúscula a união maiúscula b; fecha parênteses; interseção maiúscula c; é igual a; abre parênteses, maiúscula a interseção maiúscula c; fecha parênteses; união; abre parênteses, maiúscula b interseção maiúscula c; fecha parênteses |
| SimpleSpeak | Verbose | abre parênteses, maiúscula a união maiúscula b; fecha parênteses; interseção maiúscula c; é igual a; abre parênteses, maiúscula a interseção maiúscula c; fecha parênteses; união; abre parênteses, maiúscula b interseção maiúscula c; fecha parênteses |

### 8. Implicação

`p \Rightarrow q \equiv \neg p \lor q`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>p</mi><mo>&#x021D2;</mo><mi>q</mi><mo>&#x02261;</mo><mi>&#x000AC;</mi><mi>p</mi><mo>&#x02228;</mo><mi>q</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | p implica que q, idêntico a, não p ou q |
| ClearSpeak | Medium | p implica que q, é idêntico a, não p ou q |
| ClearSpeak | Verbose | p implica que q, é idêntico a, não p ou q |
| SimpleSpeak | Terse | p implica que q, idêntico a, não p ou q |
| SimpleSpeak | Medium | p implica que q, é idêntico a, não p ou q |
| SimpleSpeak | Verbose | p implica que q, é idêntico a, não p ou q |

### 9. Quantificadores

`\forall \varepsilon > 0 \; \exists \delta > 0`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo>&#x02200;</mo><mi>&#x003B5;</mi><mo>&#x0003E;</mo><mn>0</mn><mspace width="0.278em" /><mo>&#x02203;</mo><mi>&#x003B4;</mi><mo>&#x0003E;</mo><mn>0</mn></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | para todo; épsilon maior que, 0, existe delta maior que 0 |
| ClearSpeak | Medium | para todo; épsilon é maior que, 0, existe delta é maior que 0 |
| ClearSpeak | Verbose | para todo; épsilon é maior que, 0, existe delta é maior que 0 |
| SimpleSpeak | Terse | para todo; épsilon maior que, 0, existe delta maior que 0 |
| SimpleSpeak | Medium | para todo; épsilon é maior que, 0, existe delta é maior que 0 |
| SimpleSpeak | Verbose | para todo; épsilon é maior que, 0, existe delta é maior que 0 |

### 10. Continência

`\emptyset \subseteq A \subseteq \mathbb{N}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo>&#x02205;</mo><mo>&#x02286;</mo><mi>A</mi><mo>&#x02286;</mo><mi>&#x02115;</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | conjunto vazio, contido em ou é igual a, maiúscula a, contido em ou é igual a, números naturais |
| ClearSpeak | Medium | conjunto vazio, está contido em ou é igual a, maiúscula a, está contido em ou é igual a, números naturais |
| ClearSpeak | Verbose | conjunto vazio, está contido em ou é igual a, maiúscula a, está contido em ou é igual a, números naturais |
| SimpleSpeak | Terse | conjunto vazio, contido em ou é igual a, maiúscula a, contido em ou é igual a, números naturais |
| SimpleSpeak | Medium | conjunto vazio, está contido em ou é igual a, maiúscula a, está contido em ou é igual a, números naturais |
| SimpleSpeak | Verbose | conjunto vazio, está contido em ou é igual a, maiúscula a, está contido em ou é igual a, números naturais |


## Cálculo

### 11. Limite fundamental

`\lim_{x \to 0} \frac{\sin x}{x} = 1`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><msub><mo>lim</mo><mrow><mi>x</mi><mo>&#x02192;</mo><mn>0</mn></mrow></msub><mfrac><mrow><mi>sin</mi><mi>x</mi></mrow><mrow><mi>x</mi></mrow></mfrac><mo>&#x0003D;</mo><mn>1</mn></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | limite quando x tende a 0; sen x sobre x; igual a 1 |
| ClearSpeak | Medium | limite quando x tende a 0, de seno x sobre x; é igual a 1 |
| ClearSpeak | Verbose | limite quando x tende a 0, de, seno x sobre x, fim da fração; é igual a 1 |
| SimpleSpeak | Terse | limite quando x tende a 0; fração, sen x, sobre x, fim da fração; igual a 1 |
| SimpleSpeak | Medium | limite quando x tende a 0, de, fração, seno de x, sobre x, fim da fração; é igual a 1 |
| SimpleSpeak | Verbose | limite quando x tende a 0, de, fração, seno de x, sobre x, fim da fração; é igual a 1 |

### 12. Derivada

`\frac{d}{dx}\left(x^{2} e^{x}\right) = 2x e^{x} + x^{2} e^{x}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mfrac><mrow><mi>d</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mrow><mo stretchy="true" fence="true" form="prefix">&#x00028;</mo><msup><mi>x</mi><mrow><mn>2</mn></mrow></msup><msup><mi>e</mi><mrow><mi>x</mi></mrow></msup><mo stretchy="true" fence="true" form="postfix">&#x00029;</mo></mrow><mo>&#x0003D;</mo><mn>2</mn><mi>x</mi><msup><mi>e</mi><mrow><mi>x</mi></mrow></msup><mo>&#x0002B;</mo><msup><mi>x</mi><mrow><mn>2</mn></mrow></msup><msup><mi>e</mi><mrow><mi>x</mi></mrow></msup></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | d sobre d x, vezes; abre parênteses, x ao quadrado e elevado a x; fecha parênteses; igual a; 2 x e elevado a x, mais, x ao quadrado e elevado a x |
| ClearSpeak | Medium | d sobre d x, vezes; abre parênteses, x ao quadrado e elevado a x; fecha parênteses; é igual a; 2 x e elevado a x, mais, x ao quadrado e elevado a x |
| ClearSpeak | Verbose | d sobre d x, fim da fração; multiplicado por; abre parênteses, x ao quadrado e elevado a x; fecha parênteses; é igual a; 2 x e elevado a x, mais, x ao quadrado e elevado a x |
| SimpleSpeak | Terse | fração, d sobre, d x, fim da fração; vezes; abre parênteses, x ao quadrado e elevado a x; fecha parênteses; igual a; 2 x e elevado a x, mais, x ao quadrado e elevado a x |
| SimpleSpeak | Medium | fração, d sobre, d x, fim da fração; vezes; abre parênteses, x ao quadrado e elevado a x; fecha parênteses; é igual a; 2 x e elevado a x, mais, x ao quadrado e elevado a x |
| SimpleSpeak | Verbose | fração, d sobre, d x, fim da fração; multiplicado por; abre parênteses, x elevado ao quadrado, e elevado a x; fecha parênteses; é igual a; 2 x e elevado a x, mais, x elevado ao quadrado, e elevado a x |

### 13. Integral definida

`\int_{0}^{1} x^{2}\,dx = \frac{1}{3}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><msubsup><mo>&#x0222B;</mo><mrow><mn>0</mn></mrow><mrow><mn>1</mn></mrow></msubsup><msup><mi>x</mi><mrow><mn>2</mn></mrow></msup><mspace width="0.167em" /><mi>d</mi><mi>x</mi><mo>&#x0003D;</mo><mfrac><mrow><mn>1</mn></mrow><mrow><mn>3</mn></mrow></mfrac></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | integral de 0 a 1 de, x ao quadrado d x; igual a 1 terço |
| ClearSpeak | Medium | a integral de 0 a 1 de, x ao quadrado d x; é igual a 1 terço |
| ClearSpeak | Verbose | a integral de 0 a 1 de, x ao quadrado d x; é igual a 1 terço |
| SimpleSpeak | Terse | integral de 0 a 1 de, x ao quadrado d x; igual a 1 terço |
| SimpleSpeak | Medium | a integral de 0 a 1 de, x ao quadrado d x; é igual a 1 terço |
| SimpleSpeak | Verbose | a integral de 0 a 1 de, x elevado ao quadrado, d x; é igual a 1 terço |

### 14. Série

`\sum_{n=1}^{\infty} \frac{1}{n^{2}} = \frac{\pi^{2}}{6}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><msubsup><mo>&#x02211;</mo><mrow><mi>n</mi><mo>&#x0003D;</mo><mn>1</mn></mrow><mrow><mo>&#x0221E;</mo></mrow></msubsup><mfrac><mrow><mn>1</mn></mrow><mrow><msup><mi>n</mi><mrow><mn>2</mn></mrow></msup></mrow></mfrac><mo>&#x0003D;</mo><mfrac><mrow><msup><mi>&#x003C0;</mi><mrow><mn>2</mn></mrow></msup></mrow><mrow><mn>6</mn></mrow></mfrac></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | somatório de n igual a 1 a infinito de; a fração com numerador 1; e denominador n ao quadrado; igual a, a fração com numerador; pi ao quadrado; e denominador 6 |
| ClearSpeak | Medium | o somatório de n é igual a 1 a infinito de; a fração com numerador 1; e denominador n ao quadrado; é igual a; a fração com numerador; pi ao quadrado; e denominador 6 |
| ClearSpeak | Verbose | o somatório de n é igual a 1 a infinito de; a fração com numerador 1; e denominador n ao quadrado; fim da fração; é igual a; a fração com numerador; pi ao quadrado; e denominador 6; fim da fração |
| SimpleSpeak | Terse | somatório de n igual a 1 a infinito de; fração, 1 sobre, n ao quadrado, fim da fração; igual a, fração, pi ao quadrado, sobre 6, fim da fração |
| SimpleSpeak | Medium | o somatório de n é igual a 1 a infinito de; fração, 1 sobre, n ao quadrado, fim da fração; é igual a, fração, pi ao quadrado, sobre 6, fim da fração |
| SimpleSpeak | Verbose | o somatório de n é igual a 1 a infinito de; fração, 1 sobre, n elevado ao quadrado, fim da fração; é igual a, fração, pi elevado ao quadrado, sobre 6, fim da fração |

### 15. Derivada parcial

`\frac{\partial f}{\partial x} = 2xy`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mfrac><mrow><mo>&#x02202;</mo><mi>f</mi></mrow><mrow><mo>&#x02202;</mo><mi>x</mi></mrow></mfrac><mo>&#x0003D;</mo><mn>2</mn><mi>x</mi><mi>y</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | a fração com numerador; parcial f; e denominador parcial x; igual a 2 x y |
| ClearSpeak | Medium | a fração com numerador; derivada parcial f; e denominador derivada parcial x; é igual a 2 x y |
| ClearSpeak | Verbose | a fração com numerador; derivada parcial f; e denominador derivada parcial x; fim da fração; é igual a 2 x y |
| SimpleSpeak | Terse | fração, parcial f, sobre, parcial x, fim da fração; igual a 2 x y |
| SimpleSpeak | Medium | fração, derivada parcial f, sobre, derivada parcial x, fim da fração; é igual a 2 x y |
| SimpleSpeak | Verbose | fração, derivada parcial f, sobre, derivada parcial x, fim da fração; é igual a 2 x y |


## Álgebra Linear

### 16. Matriz e determinante

`A = \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}, \quad \det(A) = -2`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>A</mi><mo>&#x0003D;</mo><mo>&#x00028;</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>&#x00029;</mo><mo>&#x0002C;</mo><mspace width="1em" /><mo movablelimits="true">det</mo><mo stretchy="false">&#x00028;</mo><mi>A</mi><mo stretchy="false">&#x00029;</mo><mo>&#x0003D;</mo><mo>&#x02212;</mo><mn>2</mn></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula a igual a; a matriz 2 por 2; linha 1; 1, 2; linha 2; 3, 4; vírgula; det maiúscula a, igual a menos 2 |
| ClearSpeak | Medium | maiúscula a é igual a; a matriz 2 por 2; linha 1; 1, 2; linha 2; 3, 4; vírgula; det de maiúscula a, é igual a menos 2 |
| ClearSpeak | Verbose | maiúscula a é igual a; a matriz 2 por 2; linha 1; 1, 2; linha 2; 3, 4; fim matriz; vírgula; det de maiúscula a, é igual a menos 2 |
| SimpleSpeak | Terse | maiúscula a igual a; a matriz 2 por 2; linha 1; coluna 1; 1; coluna 2; 2; linha 2; coluna 1; 3; coluna 2; 4; vírgula; det maiúscula a, igual a menos 2 |
| SimpleSpeak | Medium | maiúscula a é igual a; a matriz 2 por 2; linha 1; coluna 1; 1; coluna 2; 2; linha 2; coluna 1; 3; coluna 2; 4; vírgula; det de maiúscula a, é igual a menos 2 |
| SimpleSpeak | Verbose | maiúscula a é igual a; a matriz 2 por 2; linha 1; coluna 1; 1; coluna 2; 2; linha 2; coluna 1; 3; coluna 2; 4; fim matriz; vírgula; det de maiúscula a, é igual a menos 2 |

### 17. Sistema

`A\mathbf{x} = \mathbf{b}, \quad \mathbf{x} \in \mathbb{R}^{n}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>A</mi><mi>&#x1D431;</mi><mo>&#x0003D;</mo><mi>&#x1D41B;</mi><mo>&#x0002C;</mo><mspace width="1em" /><mi>&#x1D431;</mi><mo>&#x02208;</mo><msup><mi>&#x0211D;</mi><mrow><mi>n</mi></mrow></msup></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula a x negrito, igual a b negrito; vírgula; x negrito pertence a números reais elevado a n |
| ClearSpeak | Medium | maiúscula a x negrito, é igual a b negrito; vírgula; x negrito pertence a números reais elevado a n |
| ClearSpeak | Verbose | maiúscula a x negrito, é igual a b negrito; vírgula; x negrito pertence a números reais elevado a n |
| SimpleSpeak | Terse | maiúscula a x negrito, igual a b negrito; vírgula; x negrito pertence a números reais elevado a n |
| SimpleSpeak | Medium | maiúscula a x negrito, é igual a b negrito; vírgula; x negrito pertence a números reais elevado a n |
| SimpleSpeak | Verbose | maiúscula a x negrito, é igual a b negrito; vírgula; x negrito pertence a números reais elevado a n |

### 18. Autovalor

`A v = \lambda v`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>A</mi><mi>v</mi><mo>&#x0003D;</mo><mi>&#x003BB;</mi><mi>v</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula a v, igual a lambda v |
| ClearSpeak | Medium | maiúscula a v, é igual a lambda v |
| ClearSpeak | Verbose | maiúscula a v, é igual a lambda v |
| SimpleSpeak | Terse | maiúscula a v, igual a lambda v |
| SimpleSpeak | Medium | maiúscula a v, é igual a lambda v |
| SimpleSpeak | Verbose | maiúscula a v, é igual a lambda v |

### 19. Produto interno e norma

`\langle u, v \rangle = \|u\| \|v\| \cos\theta`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>&#x027E8;</mi><mi>u</mi><mo>&#x0002C;</mo><mi>v</mi><mi>&#x027E9;</mi><mo>&#x0003D;</mo><mo fence="false" stretchy="false">&#x02016;</mo><mi>u</mi><mo fence="false" stretchy="false">&#x02016;</mo><mo fence="false" stretchy="false">&#x02016;</mo><mi>v</mi><mo fence="false" stretchy="false">&#x02016;</mo><mi>cos</mi><mi>&#x003B8;</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | abre colchete angular, u vírgula, v, fecha colchete angular; igual a; norma de u norma de v cos teta |
| ClearSpeak | Medium | abre colchete angular, u vírgula, v, fecha colchete angular; é igual a; norma de u norma de v cosseno teta |
| ClearSpeak | Verbose | abre colchete angular, u vírgula, v, fecha colchete angular; é igual a; a norma de u, a norma de v, cosseno teta |
| SimpleSpeak | Terse | abre colchete angular, u vírgula, v, fecha colchete angular; igual a; norma de u norma de v cos teta |
| SimpleSpeak | Medium | abre colchete angular, u vírgula, v, fecha colchete angular; é igual a; norma de u norma de v cosseno de teta |
| SimpleSpeak | Verbose | abre colchete angular, u vírgula, v, fecha colchete angular; é igual a; a norma de u, a norma de v, cosseno de teta |

### 20. Transposta e inversa

`(AB)^{T} = B^{T} A^{T}, \quad A^{-1}A = I`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo stretchy="false">&#x00028;</mo><mi>A</mi><mi>B</mi><msup><mo stretchy="false">&#x00029;</mo><mrow><mi>T</mi></mrow></msup><mo>&#x0003D;</mo><msup><mi>B</mi><mrow><mi>T</mi></mrow></msup><msup><mi>A</mi><mrow><mi>T</mi></mrow></msup><mo>&#x0002C;</mo><mspace width="1em" /><msup><mi>A</mi><mrow><mo>&#x02212;</mo><mn>1</mn></mrow></msup><mi>A</mi><mo>&#x0003D;</mo><mi>I</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | abre parênteses, maiúscula a maiúscula b; fecha parênteses transposta; igual a, maiúscula b transposta, maiúscula a transposta; vírgula; maiúscula a elevado a menos 1, maiúscula a; igual a maiúscula i |
| ClearSpeak | Medium | abre parênteses, maiúscula a maiúscula b; fecha parênteses transposta; é igual a, maiúscula b transposta, maiúscula a transposta; vírgula; maiúscula a elevado a menos 1, maiúscula a; é igual a maiúscula i |
| ClearSpeak | Verbose | abre parênteses, maiúscula a maiúscula b; fecha parênteses transposta; é igual a, maiúscula b transposta, maiúscula a transposta; vírgula; maiúscula a elevado a menos 1, maiúscula a; é igual a maiúscula i |
| SimpleSpeak | Terse | abre parênteses, maiúscula a maiúscula b; fecha parênteses transposta; igual a, maiúscula b transposta, maiúscula a transposta; vírgula; maiúscula a elevado a menos 1, maiúscula a; igual a maiúscula i |
| SimpleSpeak | Medium | abre parênteses, maiúscula a maiúscula b; fecha parênteses transposta; é igual a, maiúscula b transposta, maiúscula a transposta; vírgula; maiúscula a elevado a menos 1, maiúscula a; é igual a maiúscula i |
| SimpleSpeak | Verbose | abre parênteses, maiúscula a maiúscula b; fecha parênteses transposta; é igual a, maiúscula b transposta, maiúscula a transposta; vírgula; maiúscula a elevado a menos 1, maiúscula a; é igual a maiúscula i |


## Química

### 21. Água

`\mathrm{2H_2 + O_2 \longrightarrow 2H_2O}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mrow><mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>&#x0002B;</mo><msub><mi mathvariant="normal">O</mi><mn>2</mn></msub><mi mathvariant="normal">&#x027F6;</mi><mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></mrow></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | 2 maiúscula h, 2, mais maiúscula o, 2; forma, 2, maiúscula h, 2, maiúscula o |
| ClearSpeak | Medium | 2 maiúscula h, sub 2, mais maiúscula o, sub 2; reage formando; 2, maiúscula h, sub 2, maiúscula o |
| ClearSpeak | Verbose | 2 maiúscula h, subscrito 2; mais maiúscula o, subscrito 2; reage formando; 2, maiúscula h, subscrito 2, maiúscula o |
| SimpleSpeak | Terse | 2 maiúscula h, 2, mais maiúscula o, 2; forma, 2, maiúscula h, 2, maiúscula o |
| SimpleSpeak | Medium | 2 maiúscula h, sub 2, mais maiúscula o, sub 2; reage formando; 2, maiúscula h, sub 2, maiúscula o |
| SimpleSpeak | Verbose | 2 maiúscula h, subscrito 2; mais maiúscula o, subscrito 2; reage formando; 2, maiúscula h, subscrito 2, maiúscula o |

### 22. Ácido carbônico

`\mathrm{CO_2 + H_2O \rightleftharpoons H_2CO_3}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mrow><mi mathvariant="normal">C</mi><msub><mi mathvariant="normal">O</mi><mn>2</mn></msub><mo>&#x0002B;</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi><mo>&#x021CC;</mo><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">C</mi><msub><mi mathvariant="normal">O</mi><mn>3</mn></msub></mrow></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula c, maiúscula o, 2; mais, maiúscula h, 2, maiúscula o; está em equilíbrio com; maiúscula h, 2, maiúscula c, maiúscula o, 3 |
| ClearSpeak | Medium | maiúscula c, maiúscula o, sub 2; mais, maiúscula h, sub 2, maiúscula o; está em equilíbrio com; maiúscula h, sub 2, maiúscula c, maiúscula o, sub 3 |
| ClearSpeak | Verbose | maiúscula c, maiúscula o, subscrito 2; mais, maiúscula h, subscrito 2, maiúscula o; está em equilíbrio com; maiúscula h, subscrito 2, maiúscula c, maiúscula o, subscrito 3 |
| SimpleSpeak | Terse | maiúscula c, maiúscula o, 2; mais, maiúscula h, 2, maiúscula o; está em equilíbrio com; maiúscula h, 2, maiúscula c, maiúscula o, 3 |
| SimpleSpeak | Medium | maiúscula c, maiúscula o, sub 2; mais, maiúscula h, sub 2, maiúscula o; está em equilíbrio com; maiúscula h, sub 2, maiúscula c, maiúscula o, sub 3 |
| SimpleSpeak | Verbose | maiúscula c, maiúscula o, subscrito 2; mais, maiúscula h, subscrito 2, maiúscula o; está em equilíbrio com; maiúscula h, subscrito 2, maiúscula c, maiúscula o, subscrito 3 |

### 23. Íon

`\mathrm{Na^{+} + Cl^{-} \longrightarrow NaCl}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mrow><mi mathvariant="normal">N</mi><msup><mi mathvariant="normal">a</mi><mrow><mo>&#x0002B;</mo></mrow></msup><mo>&#x0002B;</mo><mi mathvariant="normal">C</mi><msup><mi mathvariant="normal">l</mi><mrow><mo>&#x02212;</mo></mrow></msup><mi mathvariant="normal">&#x027F6;</mi><mi mathvariant="normal">N</mi><mi mathvariant="normal">a</mi><mi mathvariant="normal">C</mi><mi mathvariant="normal">l</mi></mrow></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula n a sobrescrito mais; mais, maiúscula c l sobrescrito menos; seta longa para a direita, NaCl |
| ClearSpeak | Medium | maiúscula n a sobrescrito mais; mais, maiúscula c l sobrescrito menos; seta longa para a direita, NaCl |
| ClearSpeak | Verbose | maiúscula n a sobrescrito mais; mais, maiúscula c l sobrescrito menos; seta longa para a direita, NaCl |
| SimpleSpeak | Terse | maiúscula n a sobrescrito mais; mais, maiúscula c l sobrescrito menos; seta longa para a direita, NaCl |
| SimpleSpeak | Medium | maiúscula n a sobrescrito mais; mais, maiúscula c l sobrescrito menos; seta longa para a direita, NaCl |
| SimpleSpeak | Verbose | maiúscula n a sobrescrito mais; mais, maiúscula c l sobrescrito menos; seta longa para a direita, NaCl |

### 24. Concentração

`[\mathrm{H^{+}}] = 10^{-7}\ \mathrm{mol/L}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo stretchy="false">[</mo><mrow><msup><mi mathvariant="normal">H</mi><mrow><mo>&#x0002B;</mo></mrow></msup></mrow><mo stretchy="false">]</mo><mo>&#x0003D;</mo><msup><mn>10</mn><mrow><mo>&#x02212;</mo><mn>7</mn></mrow></msup><mtext>&#x000A0;</mtext><mrow><mi mathvariant="normal">m</mi><mi mathvariant="normal">o</mi><mi mathvariant="normal">l</mi><mo>&#x0002F;</mo><mi mathvariant="normal">L</mi></mrow></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | abre colchetes, maiúscula h sobrescrito mais; fecha colchetes; igual a; 10 elevado a menos 7; m o l barra maiúscula l |
| ClearSpeak | Medium | abre colchetes, maiúscula h sobrescrito mais; fecha colchetes; é igual a; 10 elevado a menos 7; m o l barra maiúscula l |
| ClearSpeak | Verbose | abre colchetes, maiúscula h sobrescrito mais; fecha colchetes; é igual a; 10 elevado a menos 7; m o l barra maiúscula l |
| SimpleSpeak | Terse | abre colchetes, maiúscula h sobrescrito mais; fecha colchetes; igual a; 10 elevado a menos 7; m o l barra maiúscula l |
| SimpleSpeak | Medium | abre colchetes, maiúscula h sobrescrito mais; fecha colchetes; é igual a; 10 elevado a menos 7; m o l barra maiúscula l |
| SimpleSpeak | Verbose | abre colchetes, maiúscula h sobrescrito mais; fecha colchetes; é igual a; 10 elevado a menos 7; m o l barra maiúscula l |

### 25. Isótopo

`{}^{14}_{6}\mathrm{C}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><msubsup><mrow /><mrow><mn>6</mn></mrow><mrow><mn>14</mn></mrow></msubsup><mi>&#x00043;</mi></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | 14, 6, maiúscula c |
| ClearSpeak | Medium | super 14, sub 6, maiúscula c |
| ClearSpeak | Verbose | sobrescrito 14, subscrito 6, maiúscula c |
| SimpleSpeak | Terse | 14, 6, maiúscula c |
| SimpleSpeak | Medium | super 14, sub 6, maiúscula c |
| SimpleSpeak | Verbose | sobrescrito 14, subscrito 6, maiúscula c |


## Estatística

### 26. Média amostral

`\bar{x} = \frac{1}{n} \sum_{i=1}^{n} x_{i}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mover><mrow><mi>x</mi></mrow><mo stretchy="true">&#x000AF;</mo></mover><mo>&#x0003D;</mo><mfrac><mrow><mn>1</mn></mrow><mrow><mi>n</mi></mrow></mfrac><msubsup><mo>&#x02211;</mo><mrow><mi>i</mi><mo>&#x0003D;</mo><mn>1</mn></mrow><mrow><mi>n</mi></mrow></msubsup><msub><mi>x</mi><mrow><mi>i</mi></mrow></msub></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | x barra, igual a; 1 sobre n, somatório de i igual a 1 a n de; x subscrito i |
| ClearSpeak | Medium | x barra, é igual a; 1 sobre n, o somatório de i é igual a 1 a n de; x subscrito i |
| ClearSpeak | Verbose | x barra, é igual a; 1 sobre n, fim da fração; o somatório de i é igual a 1 a n de; x subscrito i |
| SimpleSpeak | Terse | x barra, igual a; 1 sobre n; somatório de i igual a 1 a n de; x subscrito i |
| SimpleSpeak | Medium | x barra, é igual a; 1 sobre n; o somatório de i é igual a 1 a n de; x subscrito i |
| SimpleSpeak | Verbose | x barra, é igual a; 1 sobre n; o somatório de i é igual a 1 a n de; x subscrito i |

### 27. Variância

`\sigma^{2} = \frac{\sum (x_{i} - \mu)^{2}}{N}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><msup><mi>&#x003C3;</mi><mrow><mn>2</mn></mrow></msup><mo>&#x0003D;</mo><mfrac><mrow><mo>&#x02211;</mo><mo stretchy="false">&#x00028;</mo><msub><mi>x</mi><mrow><mi>i</mi></mrow></msub><mo>&#x02212;</mo><mi>&#x003BC;</mi><msup><mo stretchy="false">&#x00029;</mo><mrow><mn>2</mn></mrow></msup></mrow><mrow><mi>N</mi></mrow></mfrac></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | sigma ao quadrado igual a; a fração com numerador; somatório de abre parênteses, x subscrito i menos mi, fecha parênteses ao quadrado; e denominador maiúscula n |
| ClearSpeak | Medium | sigma ao quadrado é igual a; a fração com numerador; o somatório de abre parênteses, x subscrito i menos mi, fecha parênteses ao quadrado; e denominador maiúscula n |
| ClearSpeak | Verbose | sigma ao quadrado é igual a; a fração com numerador; o somatório de abre parênteses, x subscrito i menos mi, fecha parênteses ao quadrado; e denominador maiúscula n; fim da fração |
| SimpleSpeak | Terse | sigma ao quadrado igual a; fração, somatório de abre parênteses, x subscrito i menos mi, fecha parênteses ao quadrado, sobre maiúscula n, fim da fração |
| SimpleSpeak | Medium | sigma ao quadrado é igual a; fração, o somatório de abre parênteses, x subscrito i menos mi, fecha parênteses ao quadrado, sobre maiúscula n, fim da fração |
| SimpleSpeak | Verbose | sigma elevado ao quadrado, é igual a; fração, o somatório de abre parênteses, x subscrito i menos mi, fecha parênteses elevado ao quadrado, sobre maiúscula n, fim da fração |

### 28. Probabilidade condicional

`P(A \mid B) = \frac{P(A \cap B)}{P(B)}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>P</mi><mo stretchy="false">&#x00028;</mo><mi>A</mi><mo>&#x02223;</mo><mi>B</mi><mo stretchy="false">&#x00029;</mo><mo>&#x0003D;</mo><mfrac><mrow><mi>P</mi><mo stretchy="false">&#x00028;</mo><mi>A</mi><mo>&#x02229;</mo><mi>B</mi><mo stretchy="false">&#x00029;</mo></mrow><mrow><mi>P</mi><mo stretchy="false">&#x00028;</mo><mi>B</mi><mo stretchy="false">&#x00029;</mo></mrow></mfrac></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula p; abre parênteses, maiúscula a dado maiúscula b; fecha parênteses; igual a; a fração com numerador; maiúscula p; abre parênteses, maiúscula a interseção maiúscula b; fecha parênteses; e denominador maiúscula p maiúscula b |
| ClearSpeak | Medium | maiúscula p; abre parênteses, maiúscula a dado maiúscula b; fecha parênteses; é igual a; a fração com numerador; maiúscula p; abre parênteses, maiúscula a interseção maiúscula b; fecha parênteses; e denominador maiúscula p de maiúscula b |
| ClearSpeak | Verbose | maiúscula p; abre parênteses, maiúscula a dado maiúscula b; fecha parênteses; é igual a; a fração com numerador; maiúscula p; abre parênteses, maiúscula a interseção maiúscula b; fecha parênteses; e denominador maiúscula p de maiúscula b; fim da fração |
| SimpleSpeak | Terse | maiúscula p; abre parênteses, maiúscula a dado maiúscula b; fecha parênteses; igual a; fração, maiúscula p; abre parênteses, maiúscula a interseção maiúscula b; fecha parênteses, sobre, maiúscula p maiúscula b, fim da fração |
| SimpleSpeak | Medium | maiúscula p; abre parênteses, maiúscula a dado maiúscula b; fecha parênteses; é igual a; fração, maiúscula p; abre parênteses, maiúscula a interseção maiúscula b; fecha parênteses, sobre, maiúscula p de maiúscula b, fim da fração |
| SimpleSpeak | Verbose | maiúscula p; abre parênteses, maiúscula a dado maiúscula b; fecha parênteses; é igual a; fração, maiúscula p; abre parênteses, maiúscula a interseção maiúscula b; fecha parênteses, sobre, maiúscula p de maiúscula b, fim da fração |

### 29. Binomial

`\binom{n}{k} p^{k} (1-p)^{n-k}`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mo minsize="2.047em" maxsize="2.047em">&#x00028;</mo><mfrac linethickness="0"><mrow><mi>n</mi></mrow><mrow><mi>k</mi></mrow></mfrac><mo minsize="2.047em" maxsize="2.047em">&#x00029;</mo><msup><mi>p</mi><mrow><mi>k</mi></mrow></msup><mo stretchy="false">&#x00028;</mo><mn>1</mn><mo>&#x02212;</mo><mi>p</mi><msup><mo stretchy="false">&#x00029;</mo><mrow><mi>n</mi><mo>&#x02212;</mo><mi>k</mi></mrow></msup></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | n escolhe k p elevado a k vezes; abre parênteses, 1 menos p, fecha parênteses elevado à potência n menos k |
| ClearSpeak | Medium | n escolhe k p elevado a k vezes; abre parênteses, 1 menos p, fecha parênteses elevado à potência n menos k |
| ClearSpeak | Verbose | n escolhe k p elevado a k multiplicado por; abre parênteses, 1 menos p, fecha parênteses elevado à potência n menos k |
| SimpleSpeak | Terse | n escolhe k p elevado a k vezes; abre parênteses, 1 menos p, fecha parênteses elevado à potência n menos k |
| SimpleSpeak | Medium | n escolhe k p elevado a k vezes; abre parênteses, 1 menos p, fecha parênteses elevado à potência n menos k |
| SimpleSpeak | Verbose | n escolhe k p elevado a k multiplicado por; abre parênteses, 1 menos p, fecha parênteses elevado à potência n menos k |

### 30. Normal

`X \sim N(\mu, \sigma^{2})`

<details><summary>MathML</summary>

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML" display="inline"><mrow><mi>X</mi><mi>&#x0007E;</mi><mi>N</mi><mo stretchy="false">&#x00028;</mo><mi>&#x003BC;</mi><mo>&#x0002C;</mo><msup><mi>&#x003C3;</mi><mrow><mn>2</mn></mrow></msup><mo stretchy="false">&#x00029;</mo></mrow></math>
```
</details>

| estilo | verbosidade | fala |
|---|---|---|
| ClearSpeak | Terse | maiúscula x varia com; maiúscula n; abre parênteses, mi vírgula, sigma ao quadrado; fecha parênteses |
| ClearSpeak | Medium | maiúscula x varia com; maiúscula n de; abre parênteses, mi vírgula, sigma ao quadrado; fecha parênteses |
| ClearSpeak | Verbose | maiúscula x varia com; maiúscula n de; abre parênteses, mi vírgula, sigma ao quadrado; fecha parênteses |
| SimpleSpeak | Terse | maiúscula x varia com; maiúscula n; abre parênteses, mi vírgula, sigma ao quadrado; fecha parênteses |
| SimpleSpeak | Medium | maiúscula x varia com; maiúscula n de; abre parênteses, mi vírgula, sigma ao quadrado; fecha parênteses |
| SimpleSpeak | Verbose | maiúscula x varia com; maiúscula n de; abre parênteses, mi vírgula; sigma elevado ao quadrado; fecha parênteses |

## Registro das respostas

| # | expressão | reescreveu certo? | fração ok? | expoente ok? | picotada? | palavra estranha | faltou/sobrou |
|---|---|---|---|---|---|---|---|
| 1 | Fração mista e decimal |  |  |  |  |  |  |
| 2 | Bhaskara |  |  |  |  |  |  |
| 3 | Potência e raiz |  |  |  |  |  |  |
| 4 | Porcentagem |  |  |  |  |  |  |
| 5 | Fração de fração |  |  |  |  |  |  |
| 6 | Pertinência e compreensão |  |  |  |  |  |  |
| 7 | União e interseção |  |  |  |  |  |  |
| 8 | Implicação |  |  |  |  |  |  |
| 9 | Quantificadores |  |  |  |  |  |  |
| 10 | Continência |  |  |  |  |  |  |
| 11 | Limite fundamental |  |  |  |  |  |  |
| 12 | Derivada |  |  |  |  |  |  |
| 13 | Integral definida |  |  |  |  |  |  |
| 14 | Série |  |  |  |  |  |  |
| 15 | Derivada parcial |  |  |  |  |  |  |
| 16 | Matriz e determinante |  |  |  |  |  |  |
| 17 | Sistema |  |  |  |  |  |  |
| 18 | Autovalor |  |  |  |  |  |  |
| 19 | Produto interno e norma |  |  |  |  |  |  |
| 20 | Transposta e inversa |  |  |  |  |  |  |
| 21 | Água |  |  |  |  |  |  |
| 22 | Ácido carbônico |  |  |  |  |  |  |
| 23 | Íon |  |  |  |  |  |  |
| 24 | Concentração |  |  |  |  |  |  |
| 25 | Isótopo |  |  |  |  |  |  |
| 26 | Média amostral |  |  |  |  |  |  |
| 27 | Variância |  |  |  |  |  |  |
| 28 | Probabilidade condicional |  |  |  |  |  |  |
| 29 | Binomial |  |  |  |  |  |  |
| 30 | Normal |  |  |  |  |  |  |
