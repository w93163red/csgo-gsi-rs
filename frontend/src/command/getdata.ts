import { store } from './../store/store';
import axios from 'axios';
import { setRawData } from '../store/gameState';


export const instance = axios.create({
    baseURL: 'http://192.168.50.116:55555',
    timeout: 1000,
})

export const getState = async () => {
    const rawdata = await instance.get('/rawdata');
    console.log(rawdata.data);
    store.dispatch(setRawData({
        gameState: {},
        raw_data: rawdata.data 
    }));
}