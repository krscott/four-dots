import App from "./App.svelte"
import { AppState, appState } from "./appState"

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

window.rust_set_state = (new_state: unknown) => {
    console.log(new_state)
    appState.update(state => {
        const res = state.updateFromObject(new_state)
        if (res.isErr()) {
            console.error(res.unwrapErr())
        }
        console.log(state)
        return state
    })
}

export default app