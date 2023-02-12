import { fail, redirect } from '@sveltejs/kit'
import { PostRequest } from '../../lib/functions/RequestMixins'
import type { Action, Actions, PageServerLoad } from './$types'
import type { User, Tokens } from '../../lib/types'

export const load: PageServerLoad = async ({ locals }) => {
    // redirect user if logged in
    if (locals.user) {
        throw redirect(302, '/')
    }
}


const login: Action = async ({ cookies, request }) => {
    const data = await request.formData()
    const email = data.get('email')
    const password = data.get('password')

    if (
        typeof email !== 'string' ||
        typeof password !== 'string' ||
        !email ||
        !password
    ) {
        return fail(400, { invalid: true })
    }

    const user: User = { email: email, password: password }
    const body = { "email": user.email, "password": user.password }
    const response = await PostRequest("http://localhost/auth/login", body)

    if (response.status_code == 401) {
        return fail(400, { credentials: true, msg: response.json.message })
    }

    const tokens = response.json.data as Tokens

    cookies.set('access_token', tokens.access_token, {
        // send cookie for every page
        path: '/',
        // server side only cookie so you can't use `document.cookie`
        httpOnly: true,
        // only requests from same site can send cookies
        // https://developer.mozilla.org/en-US/docs/Glossary/CSRF
        sameSite: 'strict',
        // only sent over HTTPS in production
        secure: process.env.NODE_ENV === 'production',

    })
    cookies.set('refresh_token', tokens.refresh_token, {
        // send cookie for every page
        path: '/',
        // server side only cookie so you can't use `document.cookie`
        httpOnly: true,
        // only requests from same site can send cookies
        // https://developer.mozilla.org/en-US/docs/Glossary/CSRF
        sameSite: 'strict',
        // only sent over HTTPS in production
        secure: process.env.NODE_ENV === 'production',

    })
    throw redirect(302, '/')
}

export const actions: Actions = { login }