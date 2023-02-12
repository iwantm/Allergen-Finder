export type Tokens = {
    access_token: string,
    refresh_token: string
}

export type Resp = {
    success: Boolean,
    message: String,
    data: {}
}

export type User = {
    email: string,
    username?: string,
    password: string
}

export type UserInfo = {
    email: string,
    user_name: string,
}

export type ProductInfo = {
    barcode: string,
    product_name: string,
    ingredients: [string],
    allergens: string,
    allergensTags: [string],
    created_by: string,
    created_by_name: string,
    likes: string
}