import type { Action, Actions, PageServerLoad } from './$types'
import { PostRequest } from '../lib/functions/RequestMixins';
import type { Cookies } from '@sveltejs/kit';
import type { ProductInfo } from '../lib/types';
let product: ProductInfo

const search: Action = async ({ cookies, request }) => {
    const data = await request.formData()
    const barcode = data.get('barcode')

    const response = await PostRequest("http://localhost/product/search", { "barcode": barcode }, cookies.get("access_token"))

    const product_info = response.json.data as ProductInfo

    product = product_info

}

export const actions: Actions = { search }

export const load: PageServerLoad = async ({ request }) => {
    return { product }
} 