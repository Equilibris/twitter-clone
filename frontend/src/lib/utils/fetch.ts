import type {
	ApiResult,
	FeedError,
	CreatePostData,
	Entry,
	GetUserError,
	Me,
	PostError,
	PublicPost,
	PublicUser,
	SignInAndUpData,
	SignUpError,
	CreateCommentData,
} from '../typings/api';
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
	<T, E, params extends unknown[]>(url: string) =>
	(...params: params) =>
		get<Entry<null, T, E>>(`${url}${params.join('/')}`);

const curryPost =
	<R, T, E>(url: string) =>
	(data: R) =>
		post<Entry<R, T, E>>(url, data);

// prettier-ignore
export const paths = {
	post: {
		create: curryPost<CreatePostData, PublicPost, PostError>('/post/create'),
		feed: curryGet<ApiResult<PublicPost, null>[], PostError, [number]>('/post/feed/'),
		authorFeed: curryGet<ApiResult<PublicPost, null>[], FeedError, [string, number]>('/post/'),
		search: curryGet<ApiResult<PublicPost, null>[], FeedError, [string, number]>('/post/search/'),

		post: curryGet<PublicPost, PostError, [string]>('/post/'),

		createComment: curryPost<CreateCommentData, PublicPost, PostError>('/post/comment'),
		comments: curryGet<ApiResult<PublicPost, null>[], FeedError, [string, number]>('/post/comments/'),

		toggleLike: curryGet<PublicPost, unknown, [string]>('/post/tlike/'),
	},
	user: {
		signUp: curryPost<SignInAndUpData, Me, SignUpError>('/user/sign_up'),
		signIn: curryPost<SignInAndUpData, Me, string>('/user/sign_in'),
		signOut: curryPost<null, null, null>('/user/sign_out'),
		getUser: curryGet<PublicUser, GetUserError, [string]>('/user/get_user/'),
		getByName: curryGet<PublicUser, GetUserError, [string]>('/user/by_name/'),
		me: curryGet<Me, string, []>('/user/me'),
	},
};
