import type { Entry, Paths } from '../typings/api';
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

const get = async <entry extends Entry<null, unknown, unknown>>(
	url: string,
): Promise<entry['response']> => {
	try {
		const response = await (
			await fetch(`${SERVER_URL}${url}`, {
				// mode: 'no-cors',
				// credentials: 'include',
				headers: {
					Authorization: getRefreshToken(),
				},
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
			refresh_token: null,
		};
	}
};

const post = async <entry extends Entry<unknown, unknown, unknown>>(
	url: string,
	body: entry['request'],
): Promise<entry['response']> => {
	try {
		const response = await (
			await fetch(`${SERVER_URL}${url}`, {
				method: 'POST',
				body: JSON.stringify(body),
				// mode: 'no-cors',

				// credentials: 'include',
				headers: {
					Authorization: getRefreshToken(),
				},
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
			refresh_token: null,
		};
	}
};

const curryGet =
	<entry extends Entry<null, unknown, unknown>, params extends unknown[]>(url: string) =>
	(...params: params) =>
		get<entry>(`${url}${params.join('/')}`);

const curryPost =
	<entry extends Entry<unknown, unknown, unknown>>(url: string) =>
	(data: entry['request']) =>
		post<entry>(url, data);

export const paths = {
	post: {
		create: curryPost<Paths['post']['create']>('/post/create'),
		feed: curryGet<Paths['post']['feed'], [number]>('/post/feed/'),
	},
	user: {
		signIn: curryPost<Paths['user']['sign_in']>('/user/sign_in'),
		signUp: curryPost<Paths['user']['sign_up']>('/user/sign_up'),
		signOut: curryPost<Paths['user']['sign_out']>('/user/sign_out'),
		me: curryGet<Paths['user']['me'], []>('/user/me'),
		getUser: curryGet<Paths['user']['get_user'], [string]>('/user/get_user'),
	},
};
