import type { Action, Actions, PageServerLoad } from './$types'
import { fail, redirect } from '@sveltejs/kit';
import type { User, Tokens } from '../../lib/types';
import { PostRequest } from '../../lib/functions/RequestMixins';

export const load: PageServerLoad = async ({ locals }) => {
    if (locals.user) {
        throw redirect(302, '/')
    }
}



const register: Action = async ({ cookies, request }) => {
    const data = await request.formData()
    const username = data.get('username')
    const email = data.get('email')
    const password = data.get('password')
    if (
        typeof username !== 'string' ||
        typeof email !== 'string' ||
        typeof password !== 'string' ||
        !username ||
        !password
    ) {
        return fail(400, { invalid: true })
    }
    const user: User = { email, username, password }
    const body = { "user_name": user.username, "email": user.email, "password": user.password }
    const response = await PostRequest("http://localhost/auth/register", body)

    if (response.status_code == 406) {
        return fail(400, { fail: true, msg: response.json.message },)
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
export const actions: Actions = { register }