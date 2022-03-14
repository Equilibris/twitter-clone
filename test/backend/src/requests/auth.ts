import { check } from 'k6'
import http from 'k6/http'

export const reqSignUp = (name: string, password: string = 'password') =>
	http.post(
		'http://localhost:8000/user/sign_up',
		JSON.stringify({
			name,
			password,
		})
	)

export const signUp = () => {
	const signUp = reqSignUp(`tmp${Date.now()}-${Math.random()}`)
	check(signUp, {
		'Status is 200': (res) => res.status === 200,
	})
}
