# Translate unicode characters into the target language
# This makes use of three sources: SRE's translations, MathPlayer's translations, and Google translate.
import json
import os
import re
import sys
import time
from typing import Any, TextIO, cast

import deepl

# Using googletrans package
# from googletrans.models import Translated
# from googletrans import Translator
# GoogleTranslate = Translator(service_urls=["translate.google.us"])
# TranslatorName = "google"

_stdout = sys.stdout
if hasattr(_stdout, "reconfigure"):
    cast(Any, _stdout).reconfigure(encoding="utf-8")


# The google translate is done via https://github.com/ffreemt/google-stranslate (pip install itranslate)
# from itranslate import itranslate as translate
# TRANSLATE_URL = "https://translate.google.us"

# Old -- this fails due to gender issues (e.g, try translating "the" to Spanish).
#   People mentioned fixes, but the project seems dead
# The google translate is done via googletrans
# Note: needed to use 'pip install googletrans==4.0.0-rc1' and there is some concern this package might go away

''' In some manual testing, DeepL seems to do better than Google Translate
 deepl requires an API key.
 There is a free tier: it is limited to 500K chars/month)
 There are ~76k chars to translate (ignoring phrases with context), with ~62k of them being in unicode-full.yaml.
 So the free tier might be enough for a few languages every month.
 The lowest paid tier is $5.49/month + $25 per 1m chars. It provides access to better models.
 Below is sample DeepL code -- commented out for now.
 '''

# 1. Set up the Translator
auth_key: str | None = os.getenv("DEEPL_KEY")
DeepLTranslate: deepl.Translator = deepl.Translator(auth_key or "")
TranslatorName: str = "DeepL"

# DeepL `context` (English, not translated; see DeepL API docs) steers short strings that are ambiguous
# out of everyday English into mathematical / accessibility speech.
_DEFAULT_DEEPL_CONTEXT: str = (
    "Each segment separated by a period and newline is one English phrase spoken by assistive technology "
    "when reading mathematical notation (MathML). The text names symbols, operators, relations, set "
    "membership, limits, diacritics, or gives brief spelling cues for letters. Translate using vocabulary "
    "and phrasing common in mathematics and school-level spoken math in the target language, not colloquial "
    "conversation."
)


def _deepl_translation_context() -> str | None:
    """Return context for DeepL, or None to omit. Override with env MATHCAT_DEEPL_CONTEXT (empty disables)."""
    override = os.getenv("MATHCAT_DEEPL_CONTEXT")
    if override is not None:
        stripped = override.strip()
        return stripped if stripped else None
    return _DEFAULT_DEEPL_CONTEXT


# # 2. Define the text and target language
# text_to_translate = "Hello, world!"
# target_language = "FR" # French

# # 3. Call the translation method
# # Omitting 'source_lang' allows DeepL to auto-detect the language.
# result = DeepLTranslate.translate_text(
#     text_to_translate,
#     target_lang=target_language
# )

# # 4. Print the result
# print(f"Original Text: {text_to_translate}")
# print(f"Translated Text: {result.text}")
# print(f"Source Language Detected: {result.detected_source_lang}")
# print(f"Billed chars: {result.billed_characters}")

# Google allows up to 500K chars translation/month, so using a key likely would be free anyway

# Got blocked trying to translated unicode-full.yaml (~8,100 text strings).
# Some sites said ~450 requests in a short time will causes a block
# Some other places said there is ~5,000 char limit to a block request.
# There are ~15 chars/request on average
# To speed things up and avoid getting blocked, two passes are taken:
# 1. Go through the document and find all strings to translate and add them to a dictionary.
# 2. Convert the dictionary to a list, break it into appropriate chunks, and translate each chunk, building up a new dictionary
# 3. Process the file and decide on using MP, SRE, or Google for the text; add comments for the others
# This results in unneeded translations because if MP and SRE agree, we shouldn't bother with the translation.
#  However, knowing what MP and SRE will use for translations requires knowing the 'ch' and that's more work.
# The solution I've adopted is a bit ugly in that the two passes have some duplication.


def translate_char(
    ch: str,
    ignore_ch: bool,
    en_text: str,
    mathplayer: dict[str, str],
    sre: dict[str, str],
    access8: dict[str, str],
    translator: dict[str, str],
) -> tuple[str, str, str, str]:
    mp_trans = mathplayer.get(ch, '')
    sre_trans = sre.get(ch, '')
    access8_trans = access8.get(ch, '')
    # don't bother do the translation if mp and sre agree
    translator_trans = ''
    # print(f"mp_trans/sre_trans/access8_trans: '{mp_trans}/{sre_trans}/{access8_trans}'")
    if ignore_ch or mp_trans != sre_trans or mp_trans == '':
        en_text = en_text.replace("eigh", "a").replace("Eigh", "A").replace("cap", "uppercase").replace("paren", "parenthesis")
        if ignore_ch:
            mp_trans = ''
            sre_trans = ''
            access8_trans = ''
        translator_trans = translator[en_text] if len(en_text) > 1 else ch
        # print("Google Translation:('{}') = '{}'".format(en_text, google_trans))

    return (mp_trans, sre_trans, access8_trans, translator_trans)


TextToTranslate: re.Pattern[str] = re.compile('t: "([^"]+)"')


def translate_char_line(
    ch: str,
    line: str,
    mathplayer: dict[str, str],
    sre: dict[str, str],
    access8: dict[str, str],
    translator: dict[str, str],
) -> tuple[str, dict[str, Any]]:
    # using a closure for this is ugly
    result: dict[str, Any] = {}

    def do_translate_char(match_obj: re.Match[str]) -> str:
        if match_obj:
            alternatives = []
            ignore_ch = line.find('then:') >= 0   # usually an alternative to what mp and sre would say
            mp_trans, sre_trans, access8_trans, translator_trans = translate_char(
                ch, ignore_ch, match_obj.group(1), mathplayer, sre, access8, translator
            )
            # print("ch='{}', mp/sre/access8/translator='{}'/'{}'/'{}'/'{}'\n"
            #       .format(ch, mp_trans, sre_trans, access8_trans, translator_trans))
            if mp_trans == '' and sre_trans == '' and access8_trans == '' and translator_trans == '':
                translation = match_obj.group(1)       # found nothing (can this happen?)
            elif mp_trans == '' and sre_trans == '' and access8_trans == '':
                translation = translator_trans
                alternatives.append(f"{TranslatorName} translation")
            elif mp_trans == sre_trans and mp_trans != '':
                translation = mp_trans      # skip the translator translation because mp and sre agree
            elif translator_trans == mp_trans and translator_trans != '':
                translation = mp_trans
                if sre_trans:
                    alternatives.append(f"SRE: '{sre_trans}'")
            elif translator_trans == sre_trans and translator_trans != '':
                translation = sre_trans
                if mp_trans:
                    alternatives.append(f"MathPlayer: '{mp_trans}'")
            # at this point we know none of the options match and that at least one of mp or sre is non-empty
            elif sre_trans:
                translation = sre_trans
                if mp_trans:
                    alternatives.append(f"MathPlayer: '{mp_trans}'")
                alternatives.append(f"{TranslatorName}: '{translator_trans}'")
            elif mp_trans:
                translation = mp_trans
                if sre_trans:
                    alternatives.append(f"SRE: '{sre_trans}'")
                alternatives.append(f"{TranslatorName}: '{translator_trans}'")
            elif access8_trans:
                translation = access8_trans
            else:       # only translation comes from translator
                translation = translator_trans

            result['original'] = match_obj.group(1)
            result['translation'] = translation
            result['alternatives'] = alternatives
            if line.find('divided by') != -1:
                print(f"  divided by translation: {translation}")
            return f't: "{translation}"'
        else:
            return line
    return (line if line.lstrip().startswith('#') else TextToTranslate.sub(do_translate_char, line),  result)

# char defs take one of two forms:
# single line: - "̇": [t: "dot above embellishment"]             # 0x307
# multiple lines:
# - "\u0391":                                     # Greek capital alpha, 0x391
#     - test:
#         if: "($Blind or $Verbosity!='Terse')"
#         then: [t: "cap"]
#     - t: alpha


CharDefStart: re.Pattern[str] = re.compile('[^-]*- "([^"])"')        # we skip ranges such as A-Z


# gather lines until a char def is found
def get_next_char_def(lines: list[str]) -> list[str]:
    iStart = 1
    while iStart < len(lines):
        if CharDefStart.match(lines[iStart]):
            return lines[:iStart]
        iStart += 1
    return lines        # last char definition


def gather_words_in_char_def(
    lines: list[str],
    lang: str,
    mathplayer: dict[str, str],
    sre: dict[str, str],
    access8: dict[str, str],
    words_to_translate: set[str],
) -> set[str]:

    def gather_words_for_text(
        ch: str,
        en_text: str,
        lang: str,
        mathplayer: dict[str, str],
        sre: dict[str, str],
        access8: dict[str, str],
        words_to_translate: set[str],
    ) -> None:
        mp_trans = mathplayer.get(ch, '')
        sre_trans = sre.get(ch, '')
        # don't bother do the translation if mp and sre agree
        # print("mp_trans/sre_trans: '{}/{}'".format(mp_trans,sre_trans))
        if mp_trans != sre_trans or mp_trans == '':     # note: ch == '' => mp_trans == ''
            en_text = (
                en_text.replace("eigh", "a").replace("Eigh", "A").replace("cap", "uppercase").replace("paren", "parenthesis")
            )
            if len(en_text) > 1:
                words_to_translate.add(en_text)

    ch_match = CharDefStart.match(lines[0])
    ch = ch_match.group(1) if ch_match else ''

    for line in lines:
        en_text = TextToTranslate.search(line)  # assumes only one "t:" per line
        if en_text:
            # if "then:" is present, it is usually an alternative to what mp and sre would say
            ch_for_line = ch if 'then:' not in line else ''
            gather_words_for_text(ch_for_line, en_text.group(1), lang, mathplayer, sre, access8, words_to_translate)

    return words_to_translate


# echo lines, substituting for any "t:"
def process_char_def(
    lines: list[str],
    mathplayer: dict[str, str],
    sre: dict[str, str],
    access8: dict[str, str],
    translator: dict[str, str],
    out_stream: TextIO,
) -> None:
    match = CharDefStart.match(lines[0])
    ch = match.group(1) if match else ''
    for line in lines:
        translated_line, details = translate_char_line(ch, line, mathplayer, sre, access8, translator)
        if translated_line:
            # make comments that don't start a line mostly align
            i_comment_char = translated_line.find('#')
            if i_comment_char > 0 and translated_line.find('"#"') >= 0:
                # avoid considering '#' in the case of it being defined: - "#": [t: "number"]
                i_comment_char = translated_line.find('#', i_comment_char+1)
            comment = ''
            if i_comment_char > 0 and not (translated_line.lstrip().startswith('#')):
                comment = translated_line[i_comment_char+1:].rstrip()
                translated_line = translated_line[:i_comment_char-1]
            if 'alternatives' in details:
                alternatives = details['alternatives']
                if details['original'] != details['translation']:
                    alternatives.insert(0, "en: '{}'".format(details['original']))
                if alternatives != []:
                    comment += '\t(' + alternatives[0]
                    for alt in alternatives[1:]:
                        comment += ", " + alt
                    comment += ')'
            if comment:
                translated_line = f"{translated_line.rstrip():<48s}\t# {comment}\n"
            # print("***{}\t# {}\n".format(translated_line.rstrip(), comment))
        out_stream.write(
            translated_line if ch else line
        )


# run over the file and figure out what words need to be translated
def collect_words_to_translate(
    file_to_translate: str,
    lang: str,
    mathplayer: dict[str, str],
    access8: dict[str, str],
    sre: dict[str, str],
) -> set[str]:
    with open(file_to_translate, encoding='utf8') as in_stream:
        lines = in_stream.readlines()
        iLine = 0
        words_to_translate: set[str] = set()
        while iLine < len(lines):
            char_def_lines = get_next_char_def(lines[iLine:])
            # print("\niLines={}\n{}".format(iLine, list(map(lambda l: l+"\n", char_def_lines))))
            if len(char_def_lines) == 0:
                break
            gather_words_in_char_def(char_def_lines, lang, mathplayer, sre, access8, words_to_translate)
            iLine += len(char_def_lines)
        return words_to_translate


# break up the words into chunks to make google translate happy (and to run faster) and return a dictionary of word: translation
MAX_CHARS_IN_CHUNK: int = 4500  # 4500 sometimes failed (language code "no")

# try to avoid google banning us
TIMEOUT: int = 2


def translate_words(source_words: set[str], lang: str) -> dict[str, str]:
    if lang == 'nb' or lang == 'nn':
        lang = 'no'  # google doesn't know those variants, but SRE uses them
    translations: dict[str, str] = {}
    deepl_context = _deepl_translation_context()

    def do_translation_chunk(chunk: set[str]) -> None:
        # translate doesn't handle a list properly -- use ".\n" to separate words
        word_string = ".\n".join(list(chunk))
        # chunk_translations = translate(words, from_lang='en', to_lang=lang, url=TRANSLATE_URL)
        # result = GoogleTranslate.translate(word_string, src='en', dest=lang)
        result = DeepLTranslate.translate_text(
            word_string,
            source_lang='EN',
            target_lang=lang.upper(),
            context=deepl_context,
        )
        text_result = result[0] if isinstance(result, list) else result
        translated_words_str = text_result.text.lower()
        # Chinese has "." translated to "。"
        translated_words_str = translated_words_str.replace('。', '.')
        translated_words = translated_words_str.split('.\n')
        if len(translated_words) != len(chunk):
            n_tr = len(translated_words)
            n_w = len(chunk)
            print(
                f"\n!!!Problem in translation: size of translations ({n_tr}) "
                f"differs from words to translate ({n_w})\n"
            )
            # The Finnish translation (at least) for some reason has a few failures where ".\n" is only "\n" (translation fails)
            # We try a last attempt by deleting the '.' and splitting at the newline
            print("Retrying by assuming '.' is missing...")
            translated_words = translated_words_str.replace('.', '').split('\n')
            if len(translated_words) != len(chunk):
                n_tr2 = len(translated_words)
                n_w2 = len(chunk)
                print(
                    f"!!!Retry failed: size of translations ({n_tr2}) "
                    f"differs from words to translate ({n_w2})\n"
                )
            print(f"Words to translate:\n{list(chunk)}")
            print(f"Translations:\n{list(translated_words)}")
        for orig, translation in zip(chunk, translated_words, strict=False):
            translations[orig] = translation

    word_list = list(source_words)
    word_list.sort()    # make deterministic for debugging
    char_count = 0
    chunk_words: set[str] = set()
    for word in word_list:
        chunk_words.add(word)
        char_count += len(word)
        if char_count >= MAX_CHARS_IN_CHUNK:
            do_translation_chunk(chunk_words)
            # print("Translated {} words...".format(len(chunk_words)))
            char_count = 0
            chunk_words = set()
            time.sleep(TIMEOUT)       # try to avoid google banning us
    do_translation_chunk(chunk_words)
    return translations


def create_new_file(
    file_to_translate: str,
    output_file: str,
    mathplayer: dict[str, str],
    sre: dict[str, str],
    access8: dict[str, str],
    translator: dict[str, str],
) -> None:
    with open(file_to_translate, encoding='utf8') as in_stream, open(output_file, 'w', encoding='utf8') as out_stream:
        lines = in_stream.readlines()
        iLine = 0
        while iLine < len(lines):
            char_def_lines = get_next_char_def(lines[iLine:])
            # print("\niLines={}\n{}".format(iLine, list(map(lambda l: l+"\n", char_def_lines))))
            if len(char_def_lines) == 0:
                break
            process_char_def(char_def_lines, mathplayer, sre, access8, translator, out_stream)
            iLine += len(char_def_lines)


def build_new_translation(path_to_mathcat: str, lang: str, unicode_file_name: str) -> None:
    sre = get_sre_unicode_dict(SRE_Location, lang)
    mathplayer = get_mathplayer_unicode_dict(MP_Location, lang)
    access8 = get_access8_unicode_dict(ACCESS8_Location, lang)

    file_lang_to_translate = lang if lang == 'vi' or lang == 'id' else 'en'      # these are already partially translated
    file_to_translate = f"{path_to_mathcat}/Rules/Languages/{file_lang_to_translate}/{unicode_file_name}.yaml"
    words_to_translate = collect_words_to_translate(file_to_translate, lang, mathplayer, access8, sre)
    translator = translate_words(words_to_translate, lang)
    print(f"Translations: MathPlayer={len(mathplayer)}, SRE={len(sre)}, Access8={len(access8)}, Translator={len(translator)}")
    create_new_file(file_to_translate, f"{lang}/{unicode_file_name}.yaml", mathplayer, sre, access8, translator)


def get_sre_unicode_dict(path: str, lang: str) -> dict[str, str]:
    try:
        char_map: dict[str, str] = {}
        full_path = path + "\\" + lang + "\\" + "symbols" + "\\"
        for filename in os.listdir(full_path):
            with open(full_path+filename, encoding='utf8') as in_stream:
                # print( "\nReading file {}".format(full_path+filename) )
                sre_data = json.load(in_stream)
                for sre_entry in sre_data:
                    # entries we care about look like {"key": "2212", "mappings": {"default": {"default": "menos"}}}
                    if "key" in sre_entry and "default" in sre_entry["mappings"]:
                        key = chr(int(sre_entry["key"], base=16))
                        if "default" in sre_entry["mappings"]["default"]:
                            char_map[key] = sre_entry["mappings"]["default"]["default"]
                        else:
                            print(f"In file {full_path+filename}: no default mapping for key={sre_entry['key']}")
        return char_map
    except OSError:
        print(f"SRE not present: lang={lang}")
        lang_parts = lang.split('-')
        return {} if len(lang_parts) == 1 else get_sre_unicode_dict(path, lang_parts[0])


# entries we care about look like char ? (unicode == 0x2212) => string{text="menos";};
# or char ? (unicode == 0x004E) => string{text= "n"+(::target_group!="Blind" ? "" : " majuscule");};;
MP_Pattern: re.Pattern[str] = re.compile(r'.*?\(unicode == 0x([0-9A-Fa-f]{4,5})\).*?"([^"]+)".*?')


def get_mathplayer_unicode_dict(path: str, lang: str) -> dict[str, str]:
    full_path = path + "\\" + lang + "\\"
    print(f"MathPlayer path='{full_path}'")
    try:
        char_map: dict[str, str] = {}
        with open(full_path+"unicode.tdl", encoding='utf8') as in_stream:
            lines = in_stream.readlines()
            print(f"  #lines={len(lines)}")
            for line in lines:
                matches = MP_Pattern.match(line)
                if matches:
                    int_key = int(matches.group(1), base=16)
                    text = matches.group(2).strip()
                    # MP makes use of char in the private use area: E000—F8FF -- don't add those
                    # Also, there's a lot of stuff in the 'zh' translation that isn't Chinese, so skip that
                    if (int_key < 0xE000 or int_key > 0xF8FF) and text and not (lang.startswith('zh') and text.isascii()):
                        key = chr(int_key)
                        char_map[key] = text
        print(f"dict entries = {len(char_map)}")
        return char_map
    except OSError:
        print(f"MathPlayer not found: lang={lang}")
        lang_parts = lang.split('-')
        return {} if len(lang_parts) == 1 else get_mathplayer_unicode_dict(path, lang_parts[0])


# entries we care about look like "∀\tfor all", where we need to make sure the first entry is a single char
Access8_Pattern: re.Pattern[str] = re.compile(r'^(.)\t(.+)$')


def get_access8_unicode_dict(path: str, lang: str) -> dict[str, str]:
    full_path = path + "\\" + lang.replace('-', '_') + "\\speech\\"
    print(f"Access8Math path='{full_path}'")
    try:
        char_map: dict[str, str] = {}
        with open(full_path+"unicode.dic", encoding='utf8') as in_stream:
            lines = in_stream.readlines()
            print(f"  #lines={len(lines)}")
            for line in lines:
                matches = Access8_Pattern.match(line)
                if matches:
                    key = matches.group(1)
                    text = matches.group(2).strip()
                    char_map[key] = text
        print(f"dict entries = {len(char_map)}")
        return char_map
    except OSError:
        print(f"Access8 not found: lang={lang}")
        lang_parts = lang.split('-')
        return {} if len(lang_parts) == 1 else get_access8_unicode_dict(path, lang_parts[0])


# for some diagnostics (from stackoverflow.com)
def dict_compare(
    lang: str,
    sre: dict[str, str],
    mp: dict[str, str],
) -> tuple[set[str], set[str], dict[str, tuple[str, str]], set[str]]:
    sre_keys = set(sre.keys())
    mp_keys = set(mp.keys())
    shared_keys = sre_keys.intersection(mp_keys)
    sre_only = sre_keys - mp_keys
    mp_only = mp_keys - sre_keys
    differ = {o: (sre[o], mp[o]) for o in shared_keys if sre[o] != mp[o]}
    same = set(o for o in shared_keys if sre[o] == mp[o])
    with open(f"diffs-{lang}.txt", 'w', encoding='utf8') as out_stream:
        def print_dict(name: str, mapping: dict[str, str] | dict[str, tuple[str, str]]) -> None:
            out_stream.write(f"\n\n---{name}---\n")
            for key in mapping:
                out_stream.write(f"  {key}({ord(key):0>4x})={mapping[key]}\n")

        def print_set(name: str, keys: set[str], orig_dict: dict[str, str]) -> None:
            out_stream.write(f"\n---{name}---\n")
            for key in keys:
                out_stream.write(f"  {key}({ord(key):0>4x})='{orig_dict[key]}'\n")

        out_stream.write(
            f"sre/mp #chars={len(sre)}/{len(mp)}, #same={len(same)}, #differ={len(differ)}, "
            f"#only sre/mp={len(sre_only)}/{len(mp_only)}"
        )
        print_dict("differ", differ)
        print_set("sre_only", sre_only, sre)
        print_set("mp_only", mp_only, mp)
    return (sre_only, mp_only, differ, same)


# Google translate fails to lots of words when I use GoogleTranslate.translate() on a single def.
#  I played with changing the quotes and a few other things. The failures shifted, but the results weren't good.
# Strangely, if you copy the '.../en/definitions.yaml' and paste that into translate.google.com, it does it all.
# Rather than waste a bunch more time on this, the code assumes you've translated the file already and stored
#   it in 'google-defs.yaml' in the current dir
# It then goes through the English version leaving the English and pulling out only the translated *values*
#   from 'google-defs.yaml' writing '[lang]-definitions.yaml'.
def translate_definitions(path_to_mathcat: str, lang: str):
    if lang == 'nb' or lang == 'nn':
        lang = 'no'  # google doesn't know those variants

    file_to_translate = f'{path_to_mathcat}/Rules/Languages/en/definitions.yaml'
    translated_file = f'{path_to_mathcat}/Rules/Languages/{lang}/definitions.yaml'
    with (
        open("google-defs.yaml", encoding='utf8') as google_stream,
        open(file_to_translate, encoding='utf8') as in_stream,
        open(translated_file, 'w', encoding='utf8') as out_stream,
    ):
        translated_lines = google_stream.readlines()
        lines = in_stream.readlines()
        i_en = 0
        i_trans = 0
        n_lines = len(lines)
        while i_en < n_lines:
            if not (lines[i_en].startswith('#')) and (lines[i_en].find(': [') >= 0 or lines[i_en].find(': {') >= 0):
                # handles 'xxx: [' inclusive of the line with the matching ']'
                (i_en, i_trans) = translate_definition(i_en, lines, i_trans, translated_lines, out_stream)
            else:
                out_stream.write(lines[i_en])
            i_en += 1
            # i_trans += 1


def translate_definition(
    i_en: int,
    lines: list[str],
    i_trans: int,
    translated_lines: list[str],
    out_stream: TextIO,
) -> tuple[int, int]:
    out_stream.write(lines[i_en])
    i_en = i_en+1    # first line is 'name: [' or 'name: {'
    # sync lines -- find '[' in translation
    while not (translated_lines[i_trans].find(': [') >= 0 or translated_lines[i_trans].find(': {') >= 0):
        i_trans += 1
    i_trans += 1   # skip the [/{ line
    while i_en < len(lines):
        if (
            translated_lines[i_en].strip().startswith('#') or
            (len(translated_lines[i_en].strip()) == 0 and len(translated_lines[i_trans].strip()) == 0)
        ):
            out_stream.write(lines[i_en])
        elif lines[i_en].find(']') >= 0 or lines[i_en].find('}') >= 0:
            out_stream.write(lines[i_en])
            return (i_en, i_trans)
        elif len(translated_lines[i_trans].strip()) == 0:  # google sometimes adds blank lines
            i_trans += 1
            continue
        else:
            print(f'en: {lines[i_en].strip()}\ntr: {translated_lines[i_trans].strip()}')
            # get indentation right
            i_spaces = lines[i_en].find('"')
            cleaned_line = (
                translated_lines[i_trans]
                .replace("“", '"').replace("”", '"').replace("„", '"').replace("、", ",")  # Chinese
                .lstrip()
            )
            out_stream.write(f'{" ".ljust(i_spaces)}' + cleaned_line)
        i_en += 1
        i_trans += 1
    return (i_en, i_trans)


def build_euro(lang: str) -> None:
    sre = get_sre_euro_dict()
    list(sre).sort()
    print(f"Translations: SRE={len(sre)}")
    with open("latex-braille-unicode.yaml", 'w', encoding='utf8') as out_stream:
        out_stream.write("---\n")
        for ch, braille in sre.items():
            if ch == '"':
                ch = '\\"'
            elif ch == '\\':
                ch = '\\\\'
            elif ch == '\\127':
                ch = '\\x7F'
            first_part = f' - "{ch}": [t: "{braille}"]'
            try:
                comment = ''
                if ch == '\\\\' or ch == '\\"':
                    comment = hex(ord(ch[1]))
                elif len(ch) == 1 or len(ch) == 2:
                    comment = hex(ord(ch))
                else:
                    comment = "0" + ch[1:]
                out_stream.write(f'{first_part:32}# {comment}\n')
            except Exception:
                print(f"failed to write a line for ch={ch}")


def get_sre_euro_dict() -> dict[str, str]:
    char_map: dict[str, str] = {}
    full_path = SRE_Location + "\\" + "euro" + "\\" + "characters" + "\\"
    for filename in os.listdir(full_path):
        if filename == "Braille.json":
            continue
        with open(full_path+filename, encoding='utf8') as in_stream:
            print(f"\nReading file {full_path+filename}")
            sre_data = json.load(in_stream)
            sre_data = sre_data[2]
            char_map.update(sre_data)
    return char_map


def write_euro_braille_file():
    with open("EuroBraille-dict.txt", 'w', encoding='utf8') as outf:
        outf.write('{\n')
        for key, value in get_sre_euro_dict().items():
            if key == '"':
                key = '\\"'
            elif key == '\\':
                key = '\\\\'
            outf.write(f'  "{key}": "{value}",\n')
        outf.write('}\n')


# if os.path.exists("unicode.yaml"):
#   os.remove("unicode.yaml")
SRE_Location: str = r"C:\Dev\speech-rule-engine\mathmaps"
MP_Location: str = r"C:\Dev\mathplayer\EqnLib\rules\pvt"
ACCESS8_Location: str = r"C:\dev\Access8Math\addon\globalPlugins\Access8Math\locale"
# (sre_only, mp_only, differ, same) = dict_compare("es", sre_chars, mp_chars)
# (sre_only, mp_only, differ, same) = dict_compare(
#     "fr",
#     get_sre_unicode_dict(SRE_Location, "fr"),
#     get_mathplayer_unicode_dict(MP_Location, "fr"),
# )
# (sre_only, mp_only, differ, same) = dict_compare(
#     "it",
#     get_sre_unicode_dict(SRE_Location, "it"),
#     get_mathplayer_unicode_dict(MP_Location, "it"),
# )

language: str = "fr"
build_new_translation("..", language, "unicode")
# build_new_translation("..", language, "unicode-full")

# see translate_definitions comments -- you need to manually copy the file to google translate.
# translate_definitions("..", language)
# build_euro("euro")
