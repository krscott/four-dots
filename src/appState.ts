import { writable } from "svelte/store"

import type { AppState } from "./apiTypes"

const defaultAppState = (): AppState => ({ var: "Title" })

export const appState = writable(defaultAppState())
