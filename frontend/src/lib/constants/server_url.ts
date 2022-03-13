const production = import.meta.env.PROD;

export const SERVER_URL = production
	? 'https://it1-twitter-backend.herokuapp.com'
	: 'http://127.0.0.1:8000';
