
import type { Handle } from '@sveltejs/kit'
import { GetRequest } from './lib/functions/RequestMixins'
import type { UserInfo } from './lib/types'
export const handle: Handle = async ({ event, resolve }) => {
    // get cookies from browser
    const access_token = event.cookies.get('access_token')

    if (!access_token) {
        return await resolve(event)
    }

    const response = await GetRequest("http://localhost/auth/user", access_token)
    const user = response.json.data as UserInfo

    if (response.status_code == 200) {
        event.locals.user = {
            username: user.user_name,
            email: user.email,
        }
    }

    return await resolve(event)
}
