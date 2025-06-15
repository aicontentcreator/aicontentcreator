// src/lib/backend.ts
import { invoke } from '@tauri-apps/api/core';

export type BackendResponse<T = unknown> = {
  status: 'success' | 'error';
  status_description:string;
  payload: T;
};

export async function callBackend<T = any>(
  category: string,
  payload: Record<string, any>
): Promise<BackendResponse<T>> {
  try {
    const response = await invoke<BackendResponse<T>>('handle_frontend_request', {
      req: { category, payload },
    });

    return response;
  } catch (error) {
    console.error(`[Backend Error] ${category}`, error);
    return {
      status: 'error',
      status_description: 'Failed to reach backend.',
      payload: {} as T,
    };
  }
}
