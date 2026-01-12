'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import useAuthStore from '@/store/authStore';
import Link from 'next/link';
import { Modal } from 'react-bootstrap';
import { api } from '@/lib/api';

export default function DashboardPage() {
    const router = useRouter();
    const { user, logout, initialize, isAuthenticated,updateUser } = useAuthStore();
    const [isLoading, setIsLoading] = useState(true);
    const [showModal, setShowModal] = useState(false);
    const [profileForm, setProfileForm] = useState({
        name: '',
        email: '', 
    });

    const openEditProfileModal = () => {
        setProfileForm({
            name: user.name,
            email: user.email, 
        });
        setShowModal(true);
    };

    
    useEffect(() => {
        initialize();
        setIsLoading(false);
    }, [initialize]);

    useEffect(() => {
        if (!isLoading && !isAuthenticated) {
            router.push('/login');
        }
    }, [isAuthenticated, isLoading, router]);

    const handleLogout = () => {
        logout();
        window.location.href = '/login';
    };

    const handleUpdateProfileSubmit = async () => {
        try { 
           let res = await api.updateCurrentUser(profileForm);

            // UPDATE STATE FRONTEND
            await updateUser(res.data);

            // optional: refresh user di store
            await initialize();

            // hide modal
            setShowModal(false);
        } catch (error) {
            console.error('Update profile gagal:', error);
            alert('Gagal update profile');
        }
    };


    if (!isAuthenticated) {
        return null;
    }

    if (isLoading) {
        return (
            <div className="loading-container">
                <div className="text-center">
                    <div className="spinner-border spinner-border-custom text-primary" role="status">
                        <span className="visually-hidden">Loading...</span>
                    </div>
                    <p className="mt-3 text-white fw-semibold">Loading dashboard...</p>
                </div>
            </div>
        );
    }

    if (!user) {
        return null;
    }
    return (
        <>
        <Modal
            show={showModal}
            onHide={() => setShowModal(false)}
            centered
            size="lg"
            backdrop="static" // optional
        >
            <Modal.Header closeButton>
                <Modal.Title>Edit Profile</Modal.Title>
            </Modal.Header>

            <Modal.Body>
                <div className="mb-3">
                    <label className="form-label">Name</label>
                    <input className="form-control"
                    value={profileForm.name}
                    onChange={(e) => setProfileForm({ ...profileForm, name: e.target.value })} />
                </div>
                <div className="mb-3">
                    <label className="form-label">Email</label>
                    <input className="form-control"  
                    value={profileForm.email}
                    onChange={(e) => setProfileForm({ ...profileForm, email: e.target.value }) } />
                </div> 
            </Modal.Body>

            <Modal.Footer>
                <button
                    className="btn btn-secondary"
                    onClick={() => setShowModal(false)}
                >
                    Cancel
                </button>
                <button className="btn btn-primary" onClick={() => handleUpdateProfileSubmit()}>
                    Save
                </button>
            </Modal.Footer>
        </Modal>

        <div className="dashboard-container">
            {/* Navbar */}
            <nav className="dashboard-navbar">
                <div className="container-fluid">
                    <div className="d-flex justify-content-between align-items-center py-3">
                        <div className="d-flex align-items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="white" className="bi bi-diagram-3 me-3" viewBox="0 0 16 16">
                                <path fillRule="evenodd" d="M6 3.5A1.5 1.5 0 0 1 7.5 2h1A1.5 1.5 0 0 1 10 3.5v1A1.5 1.5 0 0 1 8.5 6v1H14a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-1 0V8h-5v.5a.5.5 0 0 1-1 0V8h-5v.5a.5.5 0 0 1-1 0v-1A.5.5 0 0 1 2 7h5.5V6A1.5 1.5 0 0 1 6 4.5v-1zM8.5 5a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1zM0 11.5A1.5 1.5 0 0 1 1.5 10h1A1.5 1.5 0 0 1 4 11.5v1A1.5 1.5 0 0 1 2.5 14h-1A1.5 1.5 0 0 1 0 12.5v-1zm1.5-.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1zm4.5.5A1.5 1.5 0 0 1 7.5 10h1a1.5 1.5 0 0 1 1.5 1.5v1A1.5 1.5 0 0 1 8.5 14h-1A1.5 1.5 0 0 1 6 12.5v-1zm1.5-.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1zm4.5.5a1.5 1.5 0 0 1 1.5-1.5h1a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1-1.5 1.5h-1a1.5 1.5 0 0 1-1.5-1.5v-1zm1.5-.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1z" />
                            </svg>
                            <h3 className="text-white mb-0 fw-bold">Silsilah App</h3>
                        </div>
                        <div className="d-flex align-items-center gap-3">
                            <span className="text-white">
                                <i className="bi bi-person-circle me-2"></i>
                                {user.name}
                            </span>
                            <button
                                onClick={handleLogout}
                                className="btn btn-light btn-sm"
                            >
                                <i className="bi bi-box-arrow-right me-2"></i>
                                Logout
                            </button>
                        </div>
                    </div>
                </div>
            </nav>

            {/* Main Content */}
            <div className="container py-5">
                {/* Welcome Section */}
                <div className="row mb-4">
                    <div className="col-12">
                        <div className="card dashboard-card border-0 fade-in">
                            <div className="card-body p-4">
                                <h2 className="card-title mb-3">
                                    <i className="bi bi-house-heart me-2 text-primary"></i>
                                    Welcome, {user.name}! 👋
                                </h2>
                                <p className="text-muted mb-0">
                                    Manage your family tree and explore your heritage.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Stats Cards */}
                <div className="row g-4 mb-4">
                    <div className="col-md-4">
                        <div className="card dashboard-card border-0 text-center fade-in">
                            <div className="card-body p-4">
                                <div className="mb-3">
                                    <i className="bi bi-people-fill text-primary" style={{ fontSize: '3rem' }}></i>
                                </div>
                                <h3 className="fw-bold mb-2">0</h3>
                                <p className="text-muted mb-0">Family Members</p>
                            </div>
                        </div>
                    </div>
                    <div className="col-md-4">
                        <div className="card dashboard-card border-0 text-center fade-in" style={{ animationDelay: '0.1s' }}>
                            <div className="card-body p-4">
                                <div className="mb-3">
                                    <i className="bi bi-diagram-3 text-success" style={{ fontSize: '3rem' }}></i>
                                </div>
                                <h3 className="fw-bold mb-2">0</h3>
                                <p className="text-muted mb-0">Generations</p>
                            </div>
                        </div>
                    </div>
                    <div className="col-md-4">
                        <div className="card dashboard-card border-0 text-center fade-in" style={{ animationDelay: '0.2s' }}>
                            <div className="card-body p-4">
                                <div className="mb-3">
                                    <i className="bi bi-clock-history text-warning" style={{ fontSize: '3rem' }}></i>
                                </div>
                                <h3 className="fw-bold mb-2">0</h3>
                                <p className="text-muted mb-0">Recent Activities</p>
                            </div>
                        </div>
                    </div>
                </div>

                {/* User Profile Card */}
                <div className="row">
                    <div className="col-lg-6 mb-4">
                        <div className="card dashboard-card border-0 fade-in">
                            <div className="card-header bg-transparent border-0 pt-4">
                                <h5 className="mb-0 fw-bold">
                                    <i className="bi bi-person-badge me-2 text-primary"></i>
                                    Profile Information
                                </h5>
                            </div>
                            <div className="card-body">
                                <div className="mb-3">
                                    <span className="info-badge">Name</span>
                                    <p className="mb-0 ms-2 fs-5">{user.name}</p>
                                </div>
                                <div className="mb-3">
                                    <span className="info-badge">Email</span>
                                    <p className="mb-0 ms-2 fs-5">{user.email}</p>
                                </div>
                                <button className="btn btn-primary mt-3" onClick={() => openEditProfileModal()}>
                                    <i className="bi bi-pencil-square me-2"></i>
                                    Edit Profile
                                </button>
                            </div>
                        </div>
                    </div>

                    <div className="col-lg-6 mb-4">
                        <div className="card dashboard-card border-0 fade-in">
                            <div className="card-header bg-transparent border-0 pt-4">
                                <h5 className="mb-0 fw-bold">
                                    <i className="bi bi-shield-check me-2 text-success"></i>
                                    Security Status
                                </h5>
                            </div>
                            <div className="card-body">
                                <div className="alert alert-success border-0 mb-3">
                                    <i className="bi bi-check-circle-fill me-2"></i>
                                    Your account is secure
                                </div>
                                <div className="d-flex align-items-center justify-content-between mb-3">
                                    <div>
                                        <i className="bi bi-key-fill text-primary me-2"></i>
                                        <span>Access Token</span>
                                    </div>
                                    <span className="badge bg-success">Active</span>
                                </div>
                                <div className="d-flex align-items-center justify-content-between mb-3">
                                    <div>
                                        <i className="bi bi-arrow-repeat text-warning me-2"></i>
                                        <span>Refresh Token</span>
                                    </div>
                                    <span className="badge bg-success">Active (7 days)</span>
                                </div>
                                <small className="text-muted">
                                    <i className="bi bi-info-circle me-1"></i>
                                    Your session will auto-refresh for 7 days
                                </small>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Quick Actions */}
                <div className="row">
                    <div className="col-12">
                        <div className="card dashboard-card border-0 fade-in">
                            <div className="card-header bg-transparent border-0 pt-4">
                                <h5 className="mb-0 fw-bold">
                                    <i className="bi bi-lightning-charge me-2 text-warning"></i>
                                    Quick Actions
                                </h5>
                            </div>
                            <div className="card-body">
                                <div className="row g-3">
                                    <div className="col-md-3">
                                        <button className="btn btn-outline-primary w-100">
                                            <i className="bi bi-person-plus-fill d-block mb-2" style={{ fontSize: '2rem' }}></i>
                                            Add Member
                                        </button>
                                    </div>
                                    <div className="col-md-3">
                                        <button className="btn btn-outline-success w-100">
                                            <i className="bi bi-search d-block mb-2" style={{ fontSize: '2rem' }}></i>
                                            Search Tree
                                        </button>
                                    </div>
                                    <div className="col-md-3">
                                        <button className="btn btn-outline-info w-100">
                                            <i className="bi bi-diagram-3-fill d-block mb-2" style={{ fontSize: '2rem' }}></i>
                                            View Tree
                                        </button>
                                    </div>
                                    <div className="col-md-3">
                                        <button className="btn btn-outline-secondary w-100">
                                            <i className="bi bi-gear-fill d-block mb-2" style={{ fontSize: '2rem' }}></i>
                                            Settings
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        </div>
        
        </>
    );
}