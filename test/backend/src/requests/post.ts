import http from 'k6/http'

export const reqCreatePost = (id: string, message: string) =>
	http.post(
		'http://localhost:8000/post/create',
		JSON.stringify({
			message,
		}),
		{
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Basic ${id}`,
			},
		}
	)

export const reqPeekStack = () => http.get('http://localhost:8000/post/feed/0')

export const reqToggleLike = (id: string, post: string) =>
	http.get(`http://localhost:8000/post/tlike/${post}`, {
		headers: {
			Authorization: `Basic ${id}`,
		},
	})

export const reqCreateComment = (id: string, post: string, message: string) =>
	http.post(
		`http://localhost:8000/post/comment`,
		JSON.stringify({
			message,
			post,
		}),
		{
			headers: {
				Authorization: `Basic ${id}`,
			},
		}
	)
