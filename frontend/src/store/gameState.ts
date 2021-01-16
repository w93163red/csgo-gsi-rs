import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export interface GameState {
    gameState: any,
    raw_data: string,
}

const initialState = { gameState: {}, raw_data: {} } as GameState;

const gameStateSlice = createSlice({
    name: 'GameState',
    initialState,
    reducers: {
        setState(state, action: PayloadAction<GameState>) {
            state.gameState = action.payload.gameState;
        },
        setRawData(state, action: PayloadAction<GameState>) {
            state.raw_data = action.payload.raw_data;
        }
    },
})

export const { setState, setRawData } = gameStateSlice.actions;
export default gameStateSlice.reducer;