import { combineReducers } from '@reduxjs/toolkit';
import gamestateReducer from "./gameState";

export default combineReducers({
    gamestate: gamestateReducer 
});
