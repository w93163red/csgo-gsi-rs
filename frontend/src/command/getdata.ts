import axios from 'axios';

export const instance = axios.create({
    baseURL: 'http://localhost:55555',
    timeout: 1000,
})

export const getPlayer = async () => {
    console.log(await instance.get('/player'));
}