const API_BASE_URL: string = ''; // Use an empty string for relative paths
export const API_ENDPOINTS = {
    channels: `${API_BASE_URL}/api/channels`,
    channelAndMessages: (channelId: string) => `${API_BASE_URL}/api/channels/${channelId}`,
    messages: (channelId: string, beforeTimestamp: string | null) =>
        `${API_BASE_URL}/api/messages/${channelId}${beforeTimestamp ? `?before_msg_timestamp=${beforeTimestamp}&per_page=20` : '?per_page=20'}`,
    replies: (ts: number, userId: string, channelId: string) =>
        `${API_BASE_URL}/api/replies?ts=${ts}&user_id=${userId}&channel_id=${channelId}`,
    search: `${API_BASE_URL}/api/search`,
};
