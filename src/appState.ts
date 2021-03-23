import { writable } from "svelte/store"
import { Ok, Result } from "@hqoss/monads"

import { GameBoardState } from "./gameBoardState"
import { errStr, expectString, getUnknownProperty, expectRecord } from "./util"

export enum StateTag {
    Title = "title",
    Game = "game"
}

export class TitleState {

}

export class AppState {
    state: TitleState | GameBoardState

    constructor(state: TitleState | GameBoardState) {
        this.state = state
    }

    static default(): AppState {
        return new AppState(new TitleState)
    }

    updateFromObject(other: unknown): Result<void, Error> {
        return getUnknownProperty(other, "tag")
            .andThen(expectString)
            .andThen((tag: string): Result<void, Error> => {
                switch (tag) {
                    case StateTag.Title:
                        if (this.state instanceof TitleState) {
                            // TitleState is unary state, no need to update
                        } else {
                            this.state = new TitleState
                        }
                        return Ok(undefined)
                    case StateTag.Game:
                        {
                            if (!(this.state instanceof GameBoardState)) {
                                this.state = new GameBoardState(1, 1)
                            }

                            const gameBoardState = this.state as GameBoardState
                            return getUnknownProperty(other, "data")
                                .andThen(expectRecord)
                                .andThen(gbs => gameBoardState.updateFromRecord(gbs))
                        }
                        break

                    default:
                        return errStr("Unknown tag")
                }
            })
    }
}

export const appState = writable(AppState.default())
