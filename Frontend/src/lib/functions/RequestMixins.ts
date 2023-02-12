import fetch from "node-fetch";
import type { Resp } from "../types";
import { Headers } from "node-fetch";

export const PostRequest = async (url: string, args: {}, token?: string) => {
    const body = JSON.stringify(args)
    let headers = new Headers({ "Content-Type": "application/json" })

    if (token) {
        headers.append("Authorization", `Bearer ${token}`)
    }
    const response = await fetch(url, {
        method: 'POST', body, headers
    })
    const json = await response.json() as Resp;
    const status_code = response.status

    return { json, status_code }
}

export const GetRequest = async (url: string, token: string) => {
    const response = await fetch(url, {
        method: 'GET', headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`
        }
    })
    const json = await response.json() as Resp;
    const status_code = response.status

    return { json, status_code }
}
