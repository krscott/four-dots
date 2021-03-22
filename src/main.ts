import App from "./App.svelte"
import { State, state } from "./state"

const app = new App({
    target: document.body,
    props: {
    }
})

declare global {
    interface Window {
        rust_error_handler: (err: string) => void
        rust_set_state: (new_state: State) => void
    }
}

window.rust_error_handler = (err: string) => {
    console.error(err)
}

window.rust_set_state = (new_state: State) => {
    state.update(state => {
        Object.assign(state, new_state)
        return state
    })
}

export default app