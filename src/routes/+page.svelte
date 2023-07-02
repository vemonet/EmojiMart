<script lang="ts">
	import data from '@emoji-mart/data'
	import { Picker } from 'emoji-mart'
	import { clipboard } from '@tauri-apps/api'
	import { appWindow } from '@tauri-apps/api/window'
	import { invoke } from '@tauri-apps/api/tauri'
	import { getMatches } from '@tauri-apps/api/cli'
	import { onMount, onDestroy } from 'svelte'
	import type { EmojiData } from './+page'

	const acceptedThemes = ['auto', 'light', 'dark']
	let theme = 'auto'
	let keep = false
	let picker: any

	// Add to clipboard and close when clicking an emoji
	const onEmojiSelect = async (emoji: EmojiData) => {
		const previous = await clipboard.readText()
		clipboard.writeText(emoji.native)
		appWindow.hide()
		if (previous) await invoke('trigger_paste', { emoji: emoji.native, keep, previous: previous })
		else await invoke('trigger_paste', { emoji: emoji.native, keep: false })
		if (keep && previous) clipboard.writeText(previous)
	}

	// Close when hit <Esc>
	const handleKeypress = (event: any) => {
		if (event.code === 'Escape') {
			appWindow.close()
		}
		// TODO: else if key somewhere from A to Z, bring back focus on the picker search input?
	}

	onMount(async () => {
		let matches = await getMatches()
		if (matches.args['keep'].value?.toString().toLowerCase() === 'true') keep = true
		if (matches.args['theme'].value) theme = matches.args['theme'].value?.toString().toLowerCase()
		else {
			const sysTheme = await appWindow.theme()
			if (sysTheme) theme = sysTheme
		}
		if (acceptedThemes.indexOf(theme) === -1) {
			console.error(`The theme provided "${theme}" is not valid, we will use auto by default. Please use one of: light, dark, auto.`)
			theme = 'auto'
		}

		picker.append(new Picker({ data, onEmojiSelect, theme, autoFocus: true, dynamicWidth: true }))
		document.addEventListener('keypress', handleKeypress)
	})

	onDestroy(() => {
		document.removeEventListener('keypress', handleKeypress)
	})
</script>

<!-- https://svelte.dev/tutorial/bind-this -->
<section>
	<div bind:this={picker} />
</section>
