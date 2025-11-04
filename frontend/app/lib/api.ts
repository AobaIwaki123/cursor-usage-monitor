import type {
  UploadResponse,
  ComprehensiveStats,
  HealthResponse,
  ErrorResponse,
} from '@/app/types';

/**
 * API client for interacting with backend through Next.js API routes
 */

const API_BASE = '/api';

/**
 * Upload a CSV file
 */
export async function uploadCSV(file: File): Promise<UploadResponse> {
  const formData = new FormData();
  formData.append('csvFile', file);

  const response = await fetch(`${API_BASE}/upload`, {
    method: 'POST',
    body: formData,
  });

  const data = await response.json();

  if (!response.ok) {
    throw new Error((data as ErrorResponse).error.message);
  }

  return data as UploadResponse;
}

/**
 * Append new CSV data to existing dataset
 */
export async function appendCSV(file: File): Promise<UploadResponse> {
  const formData = new FormData();
  formData.append('csvFile', file);

  const response = await fetch(`${API_BASE}/upload/append`, {
    method: 'POST',
    body: formData,
  });

  const data = await response.json();

  if (!response.ok) {
    throw new Error((data as ErrorResponse).error.message);
  }

  return data as UploadResponse;
}

/**
 * Get comprehensive statistics
 */
export async function getComprehensiveStats(): Promise<ComprehensiveStats> {
  const response = await fetch(`${API_BASE}/stats/comprehensive`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  const data = await response.json();

  if (!response.ok) {
    throw new Error((data as ErrorResponse).error.message);
  }

  return data as ComprehensiveStats;
}

/**
 * Health check
 */
export async function checkHealth(): Promise<HealthResponse> {
  const response = await fetch(`${API_BASE}/health`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  const data = await response.json();

  if (!response.ok) {
    throw new Error((data as ErrorResponse).error.message);
  }

  return data as HealthResponse;
}
