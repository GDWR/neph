use k8s_openapi::api::core::v1::{Namespace, Node, Pod};
use kube::{Api, api::ListParams, Client};
use tauri::Window;

use crate::Settings;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn get_all_nodes() -> Vec<Node> {
    let client = Client::try_default().await.unwrap();
    let node_api: Api<Node> = Api::all(client);

    return node_api
        .list(&ListParams::default())
        .await
        .unwrap()
        .items;
}

#[tauri::command]
pub async fn get_all_pods() -> Vec<Pod> {
    let client = Client::try_default().await.unwrap();
    let node_api: Api<Pod> = Api::all(client);

    return node_api
        .list(&ListParams::default())
        .await
        .unwrap()
        .items;
}

#[tauri::command]
pub fn get_current_namespace(settings: tauri::State<Settings>) -> String {
    settings.current_namespace.lock().unwrap().to_string()
}

#[tauri::command]
pub fn set_current_namespace(new_namespace: String, settings: tauri::State<Settings>, window: Window) {
    let mut namespace = settings.current_namespace.lock().unwrap();
    *namespace = new_namespace.clone();

    window.emit("namespace-update", new_namespace).unwrap();
}

#[tauri::command]
pub async fn get_all_namespaces() -> Vec<Namespace> {
    let client = Client::try_default().await.unwrap();
    let node_api: Api<Namespace> = Api::all(client);

    return node_api
        .list(&ListParams::default())
        .await
        .unwrap()
        .items;
}

#[tauri::command]
pub async fn get_pod(uid: String) -> Pod {
    let client = Client::try_default().await.unwrap();
    let api: Api<Pod> = Api::all(client);

    return api
        .list(&ListParams::default())
        .await
        .unwrap()
        .items
        .into_iter()
        .find(|p| p.metadata.uid.as_ref() == Some(&uid))
        .unwrap();
}