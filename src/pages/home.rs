use leptos::prelude::*;
use leptos_router::components::A;
use crate::ui::atoms::{Icon, IconName};
use crate::config::path;

/// Welcome home page
///
/// Landing page with hero section, features overview, and call-to-action.
/// Uses DaisyUI classes for consistent theming.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-base-100 text-base-content">
            // Hero Section
            <section class="relative overflow-hidden">
                // Background decoration
                <div class="absolute inset-0 bg-gradient-to-br from-primary/10 via-transparent to-accent/10"></div>

                <div class="relative container mx-auto px-4 py-20 md:py-32">
                    <div class="max-w-4xl mx-auto text-center">
                        // Badge
                        <div class="inline-flex items-center gap-2 bg-primary/10 text-primary px-4 py-2 rounded-full text-sm font-medium mb-6">
                            <Icon name=IconName::TrendingUp class="w-4 h-4" />
                            "Data Visualization Platform"
                        </div>

                        // Headline
                        <h1 class="text-5xl md:text-6xl lg:text-7xl font-bold mb-6 tracking-tight">
                            <span class="text-base-content">"Transform Data into "</span>
                            <span class="text-primary">"Insights"</span>
                        </h1>

                        // Subheadline
                        <p class="text-xl md:text-2xl text-base-content/70 mb-10 max-w-2xl mx-auto">
                            "Create stunning, interactive dashboards and visualizations in minutes. No code required."
                        </p>

                        // CTA Buttons
                        <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                            <A href=path("/dashboard") attr:class="btn btn-primary btn-lg gap-2">
                                <Icon name=IconName::Add class="w-5 h-5" />
                                "Create Dashboard"
                            </A>
                            <A href=path("/projects") attr:class="btn btn-outline btn-lg gap-2">
                                <Icon name=IconName::ShowChart class="w-5 h-5" />
                                "View Projects"
                            </A>
                        </div>

                        // Stats
                        <div class="grid grid-cols-3 gap-8 mt-20 max-w-2xl mx-auto">
                            <div class="text-center">
                                <div class="text-3xl md:text-4xl font-bold text-primary mb-2">"10K+"</div>
                                <div class="text-sm text-base-content/70">Active Users</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl md:text-4xl font-bold text-accent mb-2">"50K+"</div>
                                <div class="text-sm text-base-content/70">Dashboards Created</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl md:text-4xl font-bold text-success mb-2">"99.9%"</div>
                                <div class="text-sm text-base-content/70">Uptime</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section class="py-20 bg-base-200">
                <div class="container mx-auto px-4">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl md:text-4xl font-bold mb-4">"Powerful Features"</h2>
                        <p class="text-lg text-base-content/70 max-w-2xl mx-auto">
                            "Everything you need to create, share, and collaborate on data visualizations"
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-6xl mx-auto">
                        // Feature 1
                        <div class="card bg-base-100 border border-base-300 hover:shadow-xl transition-all duration-300">
                            <div class="card-body">
                                <div class="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center mb-4">
                                    <Icon name=IconName::BarChart class="w-6 h-6 text-primary" />
                                </div>
                                <h3 class="card-title text-xl mb-2">"Interactive Charts"</h3>
                                <p class="text-base-content/70">
                                    "Choose from 50+ chart types including bar, line, pie, scatter, and more."
                                </p>
                            </div>
                        </div>

                        // Feature 2
                        <div class="card bg-base-100 border border-base-300 hover:shadow-xl transition-all duration-300">
                            <div class="card-body">
                                <div class="w-12 h-12 rounded-xl bg-accent/10 flex items-center justify-center mb-4">
                                    <Icon name=IconName::AutoAwesome class="w-6 h-6 text-accent" />
                                </div>
                                <h3 class="card-title text-xl mb-2">"AI-Powered Insights"</h3>
                                <p class="text-base-content/70">
                                    "Get automatic recommendations and insights powered by machine learning."
                                </p>
                            </div>
                        </div>

                        // Feature 3
                        <div class="card bg-base-100 border border-base-300 hover:shadow-xl transition-all duration-300">
                            <div class="card-body">
                                <div class="w-12 h-12 rounded-xl bg-success/10 flex items-center justify-center mb-4">
                                    <Icon name=IconName::Person class="w-6 h-6 text-success" />
                                </div>
                                <h3 class="card-title text-xl mb-2">"Team Collaboration"</h3>
                                <p class="text-base-content/70">
                                    "Share, comment, and collaborate on dashboards with your team in real-time."
                                </p>
                            </div>
                        </div>

                        // Feature 4
                        <div class="card bg-base-100 border border-base-300 hover:shadow-xl transition-all duration-300">
                            <div class="card-body">
                                <div class="w-12 h-12 rounded-xl bg-warning/10 flex items-center justify-center mb-4">
                                    <Icon name=IconName::Upload class="w-6 h-6 text-warning" />
                                </div>
                                <h3 class="card-title text-xl mb-2">"Easy Data Import"</h3>
                                <p class="text-base-content/70">
                                    "Connect to spreadsheets, databases, or upload CSV files in seconds."
                                </p>
                            </div>
                        </div>

                        // Feature 5
                        <div class="card bg-base-100 border border-base-300 hover:shadow-xl transition-all duration-300">
                            <div class="card-body">
                                <div class="w-12 h-12 rounded-xl bg-info/10 flex items-center justify-center mb-4">
                                    <Icon name=IconName::Shield class="w-6 h-6 text-info" />
                                </div>
                                <h3 class="card-title text-xl mb-2">"Enterprise Security"</h3>
                                <p class="text-base-content/70">
                                    "Bank-level encryption and SSO integration for data protection."
                                </p>
                            </div>
                        </div>

                        // Feature 6
                        <div class="card bg-base-100 border border-base-300 hover:shadow-xl transition-all duration-300">
                            <div class="card-body">
                                <div class="w-12 h-12 rounded-xl bg-secondary/10 flex items-center justify-center mb-4">
                                    <Icon name=IconName::Category class="w-6 h-6 text-secondary" />
                                </div>
                                <h3 class="card-title text-xl mb-2">"Customizable Templates"</h3>
                                <p class="text-base-content/70">
                                    "Start with professional templates or create your own from scratch."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // CTA Section
            <section class="py-20 bg-base-100">
                <div class="container mx-auto px-4">
                    <div class="card bg-primary text-primary-content max-w-4xl mx-auto shadow-2xl">
                        <div class="card-body text-center py-16">
                            <h2 class="card-title justify-center text-3xl md:text-4xl font-bold mb-6">
                                "Ready to Get Started?"
                            </h2>
                            <p class="text-lg text-primary-content/80 mb-8 max-w-xl mx-auto">
                                "Join thousands of teams already using DataVis Pro to visualize their data."
                            </p>
                            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                                <A href=path("/dashboard") attr:class="btn btn-lg bg-primary-content text-primary hover:bg-base-100 border-none">
                                    "Create Free Account"
                                </A>
                                <A href=path("/settings") attr:class="btn btn-lg btn-outline btn-primary-content">
                                    "Schedule Demo"
                                </A>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
