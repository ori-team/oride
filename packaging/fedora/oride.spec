Name:           oride
Version:        0.2.0
Release:        1%{?dist}
Summary:        Fast, modular and extensible terminal code editor and mini-IDE in Rust

License:        MIT
URL:            https://github.com/ori-team/oride
Source0:        https://github.com/ori-team/oride/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust >= 1.80
BuildRequires:  gcc

%description
Oride (Ori + IDE) is a fast, contained, and modular terminal code editor
and mini-IDE built in Rust. It features Vim-style modal editing, on-demand
LSP client, interactive PTY terminal, project tree with Git integration,
and a rich in-terminal Markdown previewer.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release -p oride

%check
cargo test --workspace

%install
rm -rf $RPM_BUILD_ROOT
install -d -m 0755 $RPM_BUILD_ROOT%{_bindir}
install -m 0755 target/release/oride $RPM_BUILD_ROOT%{_bindir}/oride

install -d -m 0755 $RPM_BUILD_ROOT%{_docdir}/%{name}
install -m 0644 README.md $RPM_BUILD_ROOT%{_docdir}/%{name}/
install -m 0644 assets/config.example.toml $RPM_BUILD_ROOT%{_docdir}/%{name}/

install -d -m 0755 $RPM_BUILD_ROOT%{_datadir}/licenses/%{name}
install -m 0644 LICENSE $RPM_BUILD_ROOT%{_datadir}/licenses/%{name}/

%files
%license LICENSE
%doc README.md assets/config.example.toml
%{_bindir}/oride

%changelog
* Wed Sep 10 2026 Ori Team <ori-lang@proton.me> - 0.2.0-1
- Release 0.2.0 with modal editing, task runner, and diagnostics.
