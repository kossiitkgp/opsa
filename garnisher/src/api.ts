export const API_ENDPOINTS = {
    channels: `/api/channels`,
    channelAndMessages: (channelId: string) => `/api/channels/${channelId}`,
    messages: (channelId: string, beforeTimestamp: string | null) =>
        `/api/messages/${channelId}${beforeTimestamp ? `?before_msg_timestamp=${beforeTimestamp}&per_page=20` : '?per_page=20'}`,
    replies: (ts: number, userId: string, channelId: string) =>
        `/api/replies?ts=${ts}&user_id=${userId}&channel_id=${channelId}`,
    search: `/api/search`,
};
