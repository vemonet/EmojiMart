import data from "@emoji-mart/data";
import { Picker } from 'emoji-mart'
import { clipboard } from "@tauri-apps/api";
import { listen, TauriEvent } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import { onCleanup } from 'solid-js';

export interface EmojiData {
  // Only some properties, waiting for https://github.com/missive/emoji-mart/pull/789
  id: string;
  name: string;
  colons: string;
  native: string;
}

function App(): any {

  // Add to clipboard and close when clicking an emoji
  const onEmojiSelect = (emoji: EmojiData) => {
    clipboard.writeText(emoji.native);
    setTimeout(() => appWindow.hide())
  };

  // Close when hit <Esc>
  const handleKeypress = (event: any) => {
    if (event.code === "Escape") {
      appWindow.hide()
    }
    // TODO: else focus on the picker search input?
  };
  document.addEventListener('keypress', handleKeypress);

  // Close when click out (unfortunatly also when right click)
  const focusListener = listen(TauriEvent.WINDOW_BLUR, () => {
    appWindow.hide()
  });

  onCleanup(() => {
    document.removeEventListener('keypress', handleKeypress);
    focusListener.then((unlisten) => unlisten());
  });

  return new Picker({ data, onEmojiSelect, autoFocus: true, dynamicWidth: true})
}

export default App;
