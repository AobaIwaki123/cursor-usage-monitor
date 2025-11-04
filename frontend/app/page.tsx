'use client';

import { useState } from 'react';
import FileUpload from './components/FileUpload';
import TokenUsageChart from './components/TokenUsageChart';
import CostBreakdownChart from './components/CostBreakdownChart';
import ModelStatsTable from './components/ModelStatsTable';
import ComprehensiveStats from './components/ComprehensiveStats';
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

        <div className="max-w-7xl mx-auto">
          <div className="mb-8">
            <FileUpload onFileUpload={handleFileUpload} isLoading={isLoading} />
          </div>

          {uploadedData && (
            <div className="space-y-8">
              <TokenUsageChart data={uploadedData} />
              <CostBreakdownChart data={uploadedData} />
              <ModelStatsTable data={uploadedData} />
              <ComprehensiveStats data={uploadedData} />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
