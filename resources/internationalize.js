import { readFileSync, readdirSync, writeFileSync } from 'fs'

const emojiVersion = process.argv[2] || '15'

console.log(`🚀 Generating translations for emojis version ${emojiVersion}`)

// ️Variation Selector-16
const VS16 = '\ufe0f'
// Zero Width Joiner
const ZWJ = '\u200D'
// Remove VS16 and ZWJ from emoji to compare CLDR / emoji-mart data
const strippedEmoji = (emoji) => emoji.replaceAll(VS16, '').replaceAll(ZWJ, '')

const targetFolder = 'src/data'

const capitalize = ['fr', 'es', 'de', 'cs', 'nl']

async function main() {
	// Read emoji-mart data
	let martData = JSON.parse(
		readFileSync(`emoji-mart/packages/emoji-mart-data/sets/${emojiVersion}/native.json`, 'utf-8')
	)
	// const martData = await (await fetch(`https://raw.github.com/missive/emoji-mart/main/packages/emoji-mart-data/sets/14/native.json`)).json()

	// Get i18n langs defined in emoji-mart
	const langs = []
	readdirSync('emoji-mart/packages/emoji-mart-data/i18n').forEach((file) => {
		langs.push(file.replace('.json', ''))
	})

	for (let lang of langs) {
		console.log(`✔️ Translating to ${lang}`)
		try {
			// Read CLDR data: git clone https://github.com/unicode-org/cldr-json
			const langFull = JSON.parse(
				readFileSync(
					`cldr-json/cldr-json/cldr-annotations-full/annotations/${lang}/annotations.json`,
					'utf-8'
				)
			)
			const langDerived = JSON.parse(
				readFileSync(
					`cldr-json/cldr-json/cldr-annotations-derived-full/annotationsDerived/${lang}/annotations.json`,
					'utf-8'
				)
			)
			// const langFull = await (await fetch(`https://raw.github.com/unicode-org/cldr-json/main/cldr-json/cldr-annotations-full/annotations/${lang}/annotations.json`)).json()
			// const langDerived = await (await fetch(`https://raw.github.com/unicode-org/cldr-json/main/cldr-json/cldr-annotations-derived-full/annotationsDerived/${lang}/annotations.json`)).json()

			// Combine data
			for (const emojiId in martData.emojis) {
				const emojiData = martData.emojis[emojiId]
				const emoji = strippedEmoji(emojiData.skins[0].native)

				const langFullEmoji = Object.keys(langFull.annotations.annotations).find(
					(langFullEmoji) => emoji === strippedEmoji(langFullEmoji)
				)
				if (langFullEmoji) {
					emojiData.name = langFull.annotations.annotations[langFullEmoji].tts[0]
					emojiData.keywords = langFull.annotations.annotations[langFullEmoji].default
				}

				const langDerivedEmoji = Object.keys(langDerived.annotationsDerived.annotations).find(
					(langDerivedEmoji) => emoji === strippedEmoji(langDerivedEmoji)
				)
				if (langDerivedEmoji) {
					emojiData.name = langDerived.annotationsDerived.annotations[langDerivedEmoji].tts[0]
					emojiData.keywords = langDerived.annotationsDerived.annotations[langDerivedEmoji].default
				}

				if (capitalize.includes(lang)) {
					emojiData.name = emojiData.name.charAt(0).toUpperCase() + emojiData.name.slice(1)
				}
			}

			// Write localized data
			writeFileSync(`${targetFolder}/${lang}.json`, JSON.stringify(martData))
		} catch (_err) {
			console.warn(`⚠️ Could not find cldr language file for ${lang}`)
		}
	}
}

main()

// https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-annotations-derived-full/annotationsDerived/fr/annotations.json
// https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-annotations-full/annotations/fr/annotations.json
