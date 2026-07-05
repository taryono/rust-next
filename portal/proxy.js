import { NextResponse } from 'next/server';

export function proxy(request) {
    const { pathname } = request.nextUrl;

    // Get tokens from cookies
    const accessToken = request.cookies.get('access_token')?.value;
    const refreshToken = request.cookies.get('refresh_token')?.value;

    // Check if user is authenticated
    const isAuthenticated = !!(accessToken && refreshToken);

    // Auth pages
    const authPages = ['/login', '/register'];
    const isAuthPage = authPages.includes(pathname);

    // Protected pages
    const protectedPages = ['/dashboard', '/profile'];
    const isProtectedPage = protectedPages.some(page => pathname.startsWith(page));

    // Redirect logic
    if (isProtectedPage && !isAuthenticated) {
        // Not authenticated, redirect to login
        return NextResponse.redirect(new URL('/login', request.url));
    }

    if (isAuthPage && isAuthenticated) {
        // Already authenticated, redirect to dashboard
        return NextResponse.redirect(new URL('/dashboard', request.url));
    }

    return NextResponse.next();
}

export const config = {
    matcher: [
        '/dashboard/:path*',
        '/profile/:path*',
        '/login',
        '/register',
    ],
};