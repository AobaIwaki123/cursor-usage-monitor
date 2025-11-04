import { NextRequest, NextResponse } from 'next/server';
import type { HealthResponse, ErrorResponse } from '@/app/types';

const BACKEND_URL = process.env.BACKEND_URL || 'http://api:3001';

export async function GET(request: NextRequest) {
  try {
    const response = await fetch(`${BACKEND_URL}/api/health`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    const data = await response.json();

    if (!response.ok) {
      return NextResponse.json(data as ErrorResponse, { status: response.status });
    }

    return NextResponse.json(data as HealthResponse);
  } catch (error) {
    const errorResponse: ErrorResponse = {
      success: false,
      error: {
        code: 'PROXY_ERROR',
        message: 'Failed to connect to backend API',
        details: error instanceof Error ? error.message : 'Unknown error',
      },
    };
    return NextResponse.json(errorResponse, { status: 500 });
  }
}
