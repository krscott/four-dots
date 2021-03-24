import App from "./App.svelte"
import type { AppState } from "./apiTypes"
import { appState } from "./appState"

const app = new App({
    target: document.body,
    props: {
    }
})

declare global {
    interface Window {
        rust_error_handler: (err: string) => void
        rust_set_state: (new_state: AppState) => void
    }
}

window.rust_error_handler = (err: string) => {
    console.error(err)
}

window.rust_set_state = (new_state: AppState) => {
    console.log(new_state)
    appState.update(() => new_state)
}

export default app