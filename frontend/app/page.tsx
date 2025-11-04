'use client';

import { useState } from 'react';
import FileUpload from './components/FileUpload';
import type { UploadResponse, UsageData } from './types';

export default function Home() {
  const [uploadedData, setUploadedData] = useState<UsageData[] | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const handleFileUpload = (response: UploadResponse) => {
    setUploadedData(response.data);
    setIsLoading(false);
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="container mx-auto px-4 py-8">
        <header className="mb-8">
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-2">
            Cursor Usage Dashboard
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            Upload your Cursor usage CSV file to visualize your API usage patterns and costs
          </p>
        </header>

        <div className="max-w-2xl mx-auto">
          <FileUpload onFileUpload={handleFileUpload} isLoading={isLoading} />

          {uploadedData && (
            <div className="mt-8 p-6 bg-white dark:bg-gray-800 rounded-lg shadow">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
                Upload Successful
              </h2>
              <p className="text-gray-600 dark:text-gray-400">
                Loaded {uploadedData.length} records
              </p>
              <div className="mt-4 text-sm text-gray-500 dark:text-gray-500">
                Visualization components will be added in the next tasks
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
