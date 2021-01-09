import axios from 'axios';

export const instance = axios.create({
    baseURL: 'http://192.168.50.116:55555',
    timeout: 1000,
})

export const getState = async () => {
    return await instance.get('/state');
}