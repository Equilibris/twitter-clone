import type { RequireExactlyOne } from 'type-fest';

export interface ApiResult<T, E> {
	self: string;
	data: T | null;
	error: [number, E] | null;
	refresh_token: string | null;
}

// ERRORS

export type PostError = RequireExactlyOne<{ UnknownError: string; PostDoesNotExist: string }>;
export type GetUserError = RequireExactlyOne<{
	UserDoesNotExist: string;
	UnknownError: string;
	UsernameIsNotUsed: string;
}>;
export type FeedError = RequireExactlyOne<{
	AuthorDoesNotExist: string;
	DbAccessError: string;
}>;
export type SignUpError = RequireExactlyOne<{
	UserCreation: string;
	UserDbWrite: string;
	UserAlreadyExists: string;
	FailedToCreate: string;
	BadUsername: string;
}>;
// MODELS
export interface PublicUser {
	uuid: string;
	name: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface Me extends PublicUser {}

export interface PublicPost {
	uuid: string;

	author: ApiResult<PublicUser, GetUserError>;
	message: string;

	created_at: number;
}

// DATA

export interface CreatePostData {
	message: string;
}

export interface SignInAndUpData {
	name: string;
	password: string;
}

// PATHS

export type Entry<Req, Res, ResErr> = {
	request: Req;
	response: ApiResult<Res, ResErr>;
};
