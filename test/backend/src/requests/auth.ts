import { sleep, check } from 'k6'
import http from 'k6/http'

export const signUp =
	(getInterval: () => number = () => Math.random() * 5) =>
	() => {
		sleep(getInterval())

		const signUp = http.post(
			'http://localhost:8000/user/sign_up',
			JSON.stringify({
				name: `tmp${Date.now()}-${Math.random()}`,
				password: 'password',
			}),
			{ headers: { 'Content-Type': 'application/json' } }
		)
		check(signUp, {
			'Status is 200': (res) => res.status === 200,
		})
	}
