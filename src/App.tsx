import data from "@emoji-mart/data";
import { Picker } from 'emoji-mart'
import { clipboard } from "@tauri-apps/api";
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { onCleanup } from 'solid-js';
// import { register } from '@tauri-apps/api/globalShortcut';

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
    appWindow.hide()
    invoke('trigger_paste')
  };

  // Close when hit <Esc>
  const handleKeypress = (event: any) => {
    if (event.code === "Escape") {
      appWindow.close()
    }
    // TODO: else focus on the picker search input? To overcome issue where we lose focus
  };
  document.addEventListener('keypress', handleKeypress);

  // Close when click out, also handled in main.rs, added here to try to fix issue with focus
  // const focusListener = listen(TauriEvent.WINDOW_BLUR, () => {
  //   appWindow.hide()
  // });

  // onMount( async () => {
  //   await register('Alt+Space', () => {
  //     appWindow.show()
  //     appWindow.center()
  //     appWindow.setAlwaysOnTop(true)
  //     appWindow.setFocus()
  //   });
  // });

  onCleanup(() => {
    document.removeEventListener('keypress', handleKeypress);
    // focusListener.then((unlisten) => unlisten());
  });

  return new Picker({ data, onEmojiSelect, autoFocus: true, dynamicWidth: true})
}

export default App;
