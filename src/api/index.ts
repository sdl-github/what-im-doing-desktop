import { Body, ResponseType } from "@tauri-apps/api/http"
import http from "../utils/http"

type App = {
    serialNo: string
    appName: string
    showName?: string
    appIcon?: string
}

export function asyncApp(data: App[]) {
    return http('/app', {
        method: 'POST',
        body: Body.json(data),
        responseType: ResponseType.Text,
    })
}

export function newActivity(data: App) {
    return http('/activity', {
        method: 'POST',
        body: Body.json(data),
        responseType: ResponseType.Text,
    })
}

type Device = {
    serialNo: string
    name: string
}

export function registerDevice(data: Device) {
    return http('/device', {
        method: 'POST',
        body: Body.json(data),
        responseType: ResponseType.Text,
    })
}