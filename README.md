
## 🚀 Özellikler

- **Çoklu Hesap Yönetimi (Multi-Client):** Aynı anda sınırsız sayıda hesabı yönetebilir, hepsini tek tuşla oyuna sokabilirsiniz.
- **Hesap Giriş İpucu:** Eğer uygulamanız oyuna giriş yapamıyorsa veya hata veriyorsa, hesabınızı kaydederken yazdığınız telefon numarasının başındaki "0" (sıfır) rakamını silerek veya ekleyerek tekrar deneyin.
- **Donanım Kimliği Gizleme (HWID Spoofer):** Sisteme entegre edilen `ikv_spoofer` sayesinde her hesap oyuna farklı bir donanım kimliğiyle girer. Ban riski en aza indirilir.
- **Modern Arayüz (UI):** React ve TailwindCSS ile kodlanmış, karanlık mod destekli, göz yormayan modern yönetim paneli.

---

## 🔒 Güvenlik & Veri Politikası

LimitLess, tamamen **Yerel (Local) Çalışma Prensibi**'ne dayanır. 
- Uygulama içerisine girdiğiniz karakter adları, kullanıcı bilgileri ve şifreler, şifrelenmiş (AES 256) olarak yalnızca **kendi bilgisayarınızda** (`AppData/Roaming/com.limitless.app`) yerel SQLite veritabanında saklanır. 
- Hiçbir hesap bilginiz internete veya harici bir sunucuya **asla gönderilmez.** 
- Proje açık kaynaklı olduğu için kaynak kodlarını dilediğiniz gibi inceleyebilir, dışarıya veri akışı olmadığını kendi gözlerinizle görebilirsiniz.

---

## 📥 Nasıl Kurulur ve Kullanılır?

LimitLess'i kullanmak için **iki farklı yöntem** vardır. Eğer sadece uygulamayı açıp oyuna girmek istiyorsanız 1. Yöntemi, kodları inceleyip geliştirmek istiyorsanız 2. Yöntemi seçin.

### Yöntem 1: Normal Kullanıcılar İçin (Önerilen)
Kodlarla veya derleme işlemleriyle uğraşmanıza gerek yoktur.
1. Github sayfasının sağ tarafında bulunan **Releases (Sürümler)** bölümüne tıklayın.
2. Oradaki en güncel sürümün altından **`ikvlimitless.exe`** dosyasını bilgisayarınıza indirin.
3. İndirdiğiniz EXE dosyasına çift tıklayarak uygulamayı anında kullanmaya başlayabilirsiniz!

---

### Yöntem 2: Geliştiriciler İçin (Kaynak Kodundan Derleme)

> [!WARNING]
> **ÖNEMLİ:** Bu projede Git LFS (Büyük Dosya Depolama) kullanılmaktadır. Github üzerinden yeşil renkli butona basıp **"Download ZIP" diyerek İNDİRMEYİN**. Eğer ZIP olarak indirirseniz `.exe` ve `.dll` dosyaları bozuk inecek ve uygulama çalışırken `os error 216` hatası verecektir.
> Projeyi bilgisayarınıza indirmek için **Github Desktop** kullanın veya terminalden şu komutu girin:
> `git clone https://github.com/sporter26/IKV-LimitLess.git`
> *(Not: Bu komutun çalışması için bilgisayarınızda [Git yazılımının](https://git-scm.com/download/win) kurulu olması gerekir).*

Bu uygulamayı kaynak kodundan kendi bilgisayarınızda çalıştırabilmek için aşağıdaki gereksinimleri kurmalısınız:
- **[Node.js (v18 veya üzeri)](https://nodejs.org/):** Sitesine girip "LTS" sürümünü indirerek bilgisayarınıza normal bir program gibi kurun.
- **[Rust ve Cargo](https://rustup.rs/):** Sitesinden `rustup-init.exe` dosyasını indirip çalıştırın. Açılan siyah ekranda "1" yazıp Enter'a basarak varsayılan kurulumu tamamlayın.
- **[C++ Derleme Araçları](https://visualstudio.microsoft.com/tr/visual-cpp-build-tools/):** Visual Studio Build Tools'u indirin, kurarken "C++ ile masaüstü geliştirme (Desktop development with C++)" seçeneğini işaretleyip yükleyin. (Rust'ın Windows'ta kod derleyebilmesi için şarttır).

#### Geliştirici Modunda Çalıştırma
Terminali (CMD veya PowerShell) açın ve şu kodları sırasıyla yazın:
```bash
# LimitLess klasörünün içinde sağ tıklayarak terminali aç diyin. 

# Proje klasörüne girin
cd ikvlimitless-tauri

# Kütüphaneleri indirin
npm install

# Geliştirici modunda uygulamayı başlatın
npm run tauri dev
```

---

## 📦 Nasıl .EXE Haline Getirilir? (Build)

Uygulamanın kodlarında değişiklik yaptınız ve bunu normal bir Windows programı (.exe) olarak paketlemek istiyorsunuz:

1. Terminalde `ikvlimitless-tauri` dizininde olduğunuzdan emin olun.
2. Şu komutu çalıştırın:
```bash
npm run tauri build
```
3. Derleme işlemi (bilgisayarınızın hızına göre) birkaç dakika sürebilir. Rust tüm kodları sıkıştırıp tek bir exe dosyasına gömecektir.
4. **Çıkan EXE Dosyasının Konumu:**
Derleme bittiğinde oluşturulan nihai LimitLess kurulum dosyası (veya direkt çalıştırılabilir dosya) şu klasöre gidecektir:
👉 `ikvlimitless-tauri/src-tauri/target/release/`
veya kurulum (setup) dosyası için:
👉 `ikvlimitless-tauri/src-tauri/target/release/bundle/nsis/`

Oluşan bu EXE dosyasını alıp istediğiniz bilgisayarda kullanabilirsiniz.

---

## 📜 Lisans & Yasal Uyarı
Bu yazılım tamamen yapay zeka ile açık kaynaklı ve lisanssız olarak paylaşılmaktadır. Eğitim ve oyun mekaniklerini inceleme (Reverse Engineering) amacıyla yapılmıştır. Sorumluluk tamamen kullanıcıya aittir.
