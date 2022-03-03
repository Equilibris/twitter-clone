import type { Entry } from '../typings/api';
import { SERVER_URL } from '$lib/constants/server_url';

const getRefreshToken = () =>
	typeof window !== 'undefined' && window.localStorage
		? `Basic ${localStorage.getItem('refresh_token')}`
		: undefined;

const setRefreshToken = (token: string | null) =>
	typeof window !== 'undefined' &&
	window.localStorage &&
	token &&
	localStorage.setItem('refresh_token', token);

export const get = async <entry extends Entry<unknown, unknown, unknown>>(
	url: string
): Promise<entry['response']> => {
	try {
		const response = await (
			await fetch(`${SERVER_URL}${url}`, {
				// mode: 'no-cors',
				// credentials: 'include',
				headers: {
					Authorization: getRefreshToken()
				}
			})
		).json();

		setRefreshToken(response.refresh_token);

		return response;
	} catch (error) {
		console.error(error);
		return {
			self: url,
			data: null,
			error: null,
			refresh_token: null
		};
	}
};

export const post = async <entry extends Entry<unknown, unknown, unknown>>(
	url: string,
	body: entry['request']
): Promise<entry['response']> => {
	try {
		const response = await (
			await fetch(`${SERVER_URL}${url}`, {
				method: 'POST',
				body: JSON.stringify(body),
				// mode: 'no-cors',

				// credentials: 'include',
				headers: {
					Authorization: getRefreshToken()
				}
			})
		).json();

		setRefreshToken(response.refresh_token);

		return response;
	} catch (error) {
		// TODO: Error handling
		console.error(error);
		return {
			self: url,
			data: null,
			error: null,
			refresh_token: null
		};
	}
};
