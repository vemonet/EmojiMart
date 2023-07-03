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
	let keep = true
	let picker: any
	let pickMulti = false
	let selection: string[] = []

	// Add to clipboard and close when clicking an emoji
	const onEmojiSelect = async (emoji: EmojiData) => {
		if (pickMulti) {
			selection.push(emoji.native)
		} else {
			const previous = await clipboard.readText()
			// This first write to clipboard works
			clipboard.writeText(emoji.native)
			appWindow.hide()
			if (previous) await invoke('trigger_paste', { emoji: emoji.native, keep, previous: previous })
			else await invoke('trigger_paste', { emoji: emoji.native, keep: false })
			await invoke('trigger_paste', { emoji: emoji.native, keep})

			// This last write to clipboard does not work neither!
			// if (keep && previous) clipboard.writeText(previous)
		}
	}

	const handleKeypress = (event: any) => {
		// Close when hit <Esc>
		if (event.code === 'Escape') {
			appWindow.close()
		}
		if (event.code === 'Shift' || event.shiftKey) {
			pickMulti = true
		}
	}

	onMount(async () => {
		// Get arguments
		let matches = await getMatches()
		if (matches.args['keep'].value?.toString().toLowerCase() === 'true') keep = true
		if (matches.args['theme'].value) theme = matches.args['theme'].value?.toString().toLowerCase()
		else {
			const sysTheme = await appWindow.theme()
			if (sysTheme) theme = sysTheme
		}
		if (acceptedThemes.indexOf(theme) === -1) {
			console.error(
				`The theme provided "${theme}" is not valid, we will use auto by default. Please use one of: light, dark, auto.`
			)
			theme = 'auto'
		}

		// Create the picker
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
	{#if selection.length > 0}
		<p>{selection.join(' ')}</p>
	{/if}
</section>
