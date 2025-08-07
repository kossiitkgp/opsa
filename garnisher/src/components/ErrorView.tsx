import React from 'react';

interface ErrorViewProps {
    message: string;
}

export const ErrorView: React.FC<ErrorViewProps> = ({ message }) => (
    <div className="flex-1 flex items-center justify-center text-center p-6">
        <div className="p-10 bg-red-800 rounded-2xl shadow-xl border border-red-700">
            <h1 className="text-4xl font-bold text-white mb-4">An Error Occurred</h1>
            <p className="text-red-200 text-lg">{message}</p>
        </div>
    </div>
);