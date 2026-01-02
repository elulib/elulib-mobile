/**
 * Notification Bridge for Tauri Mobile
 * 
 * This script intercepts web Notification API calls from the remote frontend
 * and forwards them to the native Tauri backend for display as native push notifications.
 * 
 * The script is injected into the webview and overrides the Notification constructor
 * to intercept all notification creation attempts.
 */

(function() {
    'use strict';
    
    // Check if Tauri is available
    if (typeof window.__TAURI_INTERNALS__ === 'undefined') {
        console.warn('[Tauri Notification Bridge] Tauri not available, using standard web notifications');
        return;
    }
    
    // Store original Notification constructor
    const OriginalNotification = window.Notification;
    
    // Get Tauri invoke function
    let invoke;
    try {
        const tauri = window.__TAURI__;
        if (tauri && tauri.tauri && tauri.tauri.invoke) {
            invoke = tauri.tauri.invoke.bind(tauri.tauri);
        } else {
            console.warn('[Tauri Notification Bridge] Tauri invoke not available');
            return;
        }
    } catch (e) {
        console.warn('[Tauri Notification Bridge] Failed to get Tauri API:', e);
        return;
    }
    
    // Override Notification constructor
    const TauriNotification = function(title, options = {}) {
        const notification = {
            title: String(title),
            body: options.body || '',
            icon: options.icon || null,
            tag: options.tag || null,
            data: options.data || null,
            requireInteraction: options.requireInteraction || false,
            silent: options.silent || false,
            timestamp: Date.now(),
        };
        
        // Show native notification via Tauri
        showNativeNotification(notification).catch(err => {
            console.error('[Tauri Notification Bridge] Failed to show native notification:', err);
            // Fallback to original notification if available
            if (OriginalNotification && OriginalNotification.permission === 'granted') {
                try {
                    new OriginalNotification(notification.title, {
                        body: notification.body,
                        icon: notification.icon
                    });
                } catch (e) {
                    console.error('[Tauri Notification Bridge] Fallback notification also failed:', e);
                }
            }
        });
        
        // Create a mock notification object for compatibility
        const mockNotification = {
            title: notification.title,
            body: notification.body,
            icon: notification.icon,
            tag: notification.tag,
            data: notification.data,
            onclick: null,
            onshow: null,
            onerror: null,
            onclose: null,
            close: function() {
                // Native notifications can't be programmatically closed
                if (this.onclose) {
                    this.onclose();
                }
            },
            addEventListener: function(event, handler) {
                if (event === 'click') {
                    this.onclick = handler;
                } else if (event === 'show') {
                    this.onshow = handler;
                } else if (event === 'error') {
                    this.onerror = handler;
                } else if (event === 'close') {
                    this.onclose = handler;
                }
            },
            removeEventListener: function(event, handler) {
                // Remove event handlers if needed
            }
        };
        
        // Trigger onshow if provided
        if (options.onshow) {
            setTimeout(() => {
                try {
                    options.onshow(mockNotification);
                } catch (e) {
                    console.error('[Tauri Notification Bridge] Error in onshow handler:', e);
                }
            }, 0);
        }
        
        return mockNotification;
    };
    
    // Copy static properties
    Object.defineProperty(TauriNotification, 'permission', {
        get: function() {
            // Check permission status (will be updated by requestPermission)
            return window.__TAURI_NOTIFICATION_PERMISSION__ || 'default';
        },
        set: function(value) {
            window.__TAURI_NOTIFICATION_PERMISSION__ = value;
        },
        configurable: true
    });
    
    Object.defineProperty(TauriNotification, 'maxActions', {
        get: function() {
            return OriginalNotification ? OriginalNotification.maxActions : 0;
        },
        configurable: true
    });
    
    // Implement requestPermission
    TauriNotification.requestPermission = function(callback) {
        const promise = invoke('request_notification_permission')
            .then(granted => {
                const permission = granted ? 'granted' : 'denied';
                window.__TAURI_NOTIFICATION_PERMISSION__ = permission;
                return permission;
            })
            .catch(err => {
                console.error('[Tauri Notification Bridge] Permission request failed:', err);
                const permission = 'denied';
                window.__TAURI_NOTIFICATION_PERMISSION__ = permission;
                return permission;
            });
        
        if (typeof callback === 'function') {
            promise.then(callback);
            return undefined;
        }
        
        return promise;
    };
    
    // Function to show native notification via Tauri
    async function showNativeNotification(notification) {
        try {
            await invoke('show_notification', {
                title: notification.title,
                body: notification.body,
                icon: notification.icon
            });
            
            console.log('[Tauri Notification Bridge] Native notification shown:', notification.title);
        } catch (error) {
            console.error('[Tauri Notification Bridge] Failed to show native notification:', error);
            throw error;
        }
    }
    
    // Replace global Notification
    window.Notification = TauriNotification;
    
    // Initialize permission status
    invoke('is_notification_supported')
        .then(supported => {
            if (supported) {
                // Request permission on initialization
                TauriNotification.requestPermission().then(permission => {
                    console.log('[Tauri Notification Bridge] Notification permission:', permission);
                });
            }
        })
        .catch(err => {
            console.warn('[Tauri Notification Bridge] Failed to check notification support:', err);
        });
    
    console.log('[Tauri Notification Bridge] Initialized successfully');
})();

