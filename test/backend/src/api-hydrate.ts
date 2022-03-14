import { Options } from 'k6/options'
import { reqSignUp } from './requests/auth'
import { check, sleep } from 'k6'
import {
	reqCreateComment,
	reqCreatePost,
	reqPeekStack,
	reqToggleLike,
} from './requests/post'
import { username } from './gen/username'
import { text } from './gen/text'

export interface ApiResult<T, E> {
	self: string
	data: T | null
	error: [number, E] | null
	refresh_token: string | null
}

export interface PublicUser {
	uuid: string
	name: string
}

export interface PublicPost {
	uuid: string

	author: ApiResult<PublicUser, null>
	message: string

	likes_count: number
	i_like: boolean

	comment: null | string
	comment_count: number

	created_at: number
}

export let options: Options = {
	vus: 1000,

	duration: '1m',

	thresholds: {
		http_req_failed: ['rate<0.01'],
		http_req_duration: ['p(95)<1000'],
		errors: ['rate<0.1'],
	},
}

const main = () => {
	const nameSeparator = '-._'[Math.floor(Math.random() * 3)]
	const uname = username().replace(/\s/g, nameSeparator)

	const signUp = reqSignUp(uname)

	check(signUp, { '[SIGNUP] Status is 200': (r) => r.status === 200 })

	if (typeof signUp.body !== 'string') return

	const signUpData: ApiResult<PublicUser, null> = JSON.parse(signUp.body)

	check(signUpData, {
		'[SIGNUP] Does not error': (r) => r.error === null,
		'[SIGNUP] Does include refresh_token': (r) => r.refresh_token !== null,
		'[SIGNUP] Echoes username': (r) => r.data?.name === uname,
	})

	const id = signUpData.refresh_token

	if (!id) return

	for (let actionIndex = 0; actionIndex < 100; actionIndex++) {
		if (Math.random() > 0.2) {
			const stack = reqPeekStack()

			check(stack, { '[COMMENT] Status is 200': (r) => r.status === 200 })

			if (typeof stack.body !== 'string') return
			try {
				const postData: ApiResult<ApiResult<PublicPost, null>[], null> =
					JSON.parse(stack.body)

				check(postData, {
					'[COMMENT] Does not error': (r) => r.error === null,
				})

				for (const i of postData.data!) {
					if (Math.random() > 0.2) {
						const like = reqToggleLike(id, i.data!.uuid)

						check(like, { 'Status is 200': (r) => r.status === 200 })
					}
					if (Math.random() > 0.7) {
						const comment = reqCreateComment(id, i.data!.uuid, text())

						check(comment, {
							'[COMMENT] Status is 200': (r) => r.status === 200,
						})
					}
				}
			} catch (error) {}
		}
		// post
		else {
			const message = text()

			const post = reqCreatePost(id, message)

			check(post, { '[POST] Status is 200': (r) => r.status === 200 })

			if (typeof post.body !== 'string') return

			const postData: ApiResult<PublicPost, null> = JSON.parse(post.body)

			check(postData, {
				'[POST] Does not error in post create': (r) => r.error === null,
				'[POST] Echoes message': (r) => r.data?.message === message,
			})
		}
		sleep(0.5 + Math.random())
	}
	sleep(0.5 + Math.random())
}

export default main
