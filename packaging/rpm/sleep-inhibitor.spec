Name:           bazzite-deck-sleep-inhibitor
Version:        1.0.0
Release:        1%{?dist}
Summary:        D-Bus sleep inhibitor daemon

License:        MIT
URL:            https://github.com/addmix/Bazzite-Deck-Sleep-Inhibitor

Source0:        %{url}/archive/refs/tags/%{version}.tar.gz#/Bazzite-Deck-Sleep-Inhibitor-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  rust-packaging
BuildRequires:  systemd-rpm-macros

Requires:       systemd
Requires:       dbus
%global debug_package %{nil}

%description
A lightweight D-Bus sleep inhibitor implementation for
desktop-less environments.

%generate_buildrequires
%cargo_generate_buildrequires

%prep
%autosetup -n Bazzite-Deck-Sleep-Inhibitor-%{version}
%cargo_prep


%build
%cargo_build


%install
install -Dm755 target/rpm/sleep_inhibitor \
    %{buildroot}%{_bindir}/sleep_inhibitor

install -Dm644 systemd/system/sleep-inhibitor-root.service \
    %{buildroot}%{_unitdir}/sleep-inhibitor-root.service

install -Dm644 systemd/user/sleep-inhibitor.service \
    %{buildroot}%{_userunitdir}/sleep-inhibitor.service

install -Dm644 dbus/com.addmix.SleepInhibitor.conf \
    %{buildroot}%{_datadir}/dbus-1/system.d/com.addmix.SleepInhibitor.conf


%files
%license LICENSE

%{_bindir}/sleep_inhibitor
%{_unitdir}/sleep-inhibitor-root.service
%{_userunitdir}/sleep-inhibitor.service
%{_datadir}/dbus-1/system.d/com.addmix.SleepInhibitor.conf

%changelog
%autochangelog