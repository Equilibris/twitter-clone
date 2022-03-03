import type { Entry } from '../typings/api';
import { SERVER_URL } from '$lib/constants/server_url';

export const get = async <entry extends Entry<unknown, unknown, unknown>>(
	url: string
): Promise<entry['response']> => {
	try {
		const response = await (
			await fetch(`${SERVER_URL}${url}`, {
				credentials: 'include'
			})
		).json();

		return response;
	} catch (error) {
		console.error(error);
		return {
			self: url,
			data: null,
			error: null
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

				credentials: 'include'
			})
		).json();

		return response;
	} catch (error) {
		console.error(error);
		return {
			self: url,
			data: null,
			error: null
		};
	}
};
