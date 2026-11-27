# Arayüz yeniden tasarım planı

Bu dosya arayüz yenilemesinin tek kaynağıdır. Her fazda önce bu dosyayı oku. Bir faz bitince "Durum" tablosunu güncelle.

## Durum

| Faz | Konu | Durum |
|---|---|---|
| 1 | Hızlı kazanımlar | tamamlandı |
| 2 | Global bağlantı durumu ve boş durumlar | bekliyor |
| 3 | "Bu hafta" ana ekranı | bekliyor |
| 4 | Takım analizi ve Oyuncular tablosu | bekliyor |
| 5 | Takvim | bekliyor |
| 6 | Cila ve kontrol | bekliyor |

## Neden değişiyor (mevcut arayüzün sorunları)

Mevcut arayüz çalışıyor ama özelliklere göre düzenlenmiş. Kullanıcının işine göre düzenlenmesi gerekiyor. Menajerin her hafta sorduğu soru şu: "Bu hafta ne yapmalıyım, uygulandı mı?" Bu sorunun cevabı şu an dört sayfaya dağılmış durumda.

1. **Genel Bakış:** 4 bileşen ve 17 faktör aynı ağırlıkta. En kritik bilgi, yani en zayıf faktör, sağ altta kayboluyor.
2. **Renk ile etiket çelişiyor:** `level_name()` 61'e "iyi" diyor, `level_color()` aynı skoru sarıya boyuyor. Neredeyse her şey sarı görünüyor ve hiçbir şey öne çıkmıyor.
3. **Bileşen adları tekrar ediyor:** hem üstteki kutucuklarda hem sütun başlıklarında var. Önceki ölçüme göre değişim de ilk bakışta görünmüyor.
4. **İç yapı kullanıcıya gösteriliyor:** Başlıktaki üç eşit buton (BepInEx / OCR / Dosyadan) kullanıcıya veri kaynağını seçtiriyor. Kullanıcı yalnızca "verileri güncelle" demek istiyor.
5. **Bağlantı durumu gömülü:** sol altta küçük bir yazı olarak duruyor. Bunun yerine her sayfada ayrı ayrı "önce bağlanın" kartları çıkıyor. Bireysel sayfasında bu kart üç kez tekrarlanıyor.
6. **Planlama:** Takvim sayfanın ana öğesi olmalı ama çipler 10–11 px. Form kontrolleri tek satırda karışık, renk açıklaması sayfanın altında satır kayıyor, plan adları şifreli ("TE 1W Bal IPS+PS").
7. **Bireysel:** Uzun paragraflar ve tekrarlanan parantez içi uyarılar var. Gerekçesi yazılmayan pasif bir birincil buton duruyor. Darboğaz, yoğunluk ve ek odak aynı oyuncular üzerinde çalıştığı halde üç ayrı kartta.
8. **Kenar çubuğunda yer kaplayan dil anahtarı:** TR/EN/NL anahtarı prime alanı kaplıyor, oysa 6 dil destekleniyor.
9. **Arka plan işi görünmüyor:** FM asistanı programı değiştirip uygulama geri yazdığında bu sessizce oluyor. Kullanıcı uygulamanın çalıştığından emin olamıyor.

**Önemli:** Mevcut düzeni ve bileşenleri korumak zorunda değilsin. Bir kalıp aşağıdaki ilkelerle çelişiyorsa değiştir. Tasarımla ilgili bir maddeye katılmıyorsan uygulamadan önce gerekçesiyle söyle.

## Tasarım ilkeleri (tüm fazlarda geçerli)

- **Menü kullanıcının haftalık döngüsüne göre:** Analiz → Plan → Uygula → Takip.
- **Renk anlam taşır:** `ACCENT` (mor) yalnızca birincil eylem ve seçili durum için. `SUCCESS` / `WARNING` / `DANGER` yalnızca durum bildirir.
- **Birincil buton:** Her sayfada en fazla bir `primary_button` olur.
- **Pasif butonlar:** Gerekçesi gösterilmeyen pasif buton kullanma. Nedeni `on_disabled_hover_text` ile yaz.
- **Açıklamalar:** Kartta tek satır kalsın, ayrıntı ⓘ tooltip'ine gitsin.
- **Rakamlar:** Skorlar sabit genişlikte ve sağa yaslı olsun.
- **Metinler:** sentence case, ünlem yok, teknik jargon yok. "BepInEx" yerine "Oyun eklentisi", "OCR" yerine "Ekrandan oku" yaz.

## Kesin kurallar

1. **Yalnızca arayüz katmanı değişir:** `app/src/gui.rs`, `theme.rs`, `individual.rs` (yalnızca çizim kodu), i18n dosyaları ve gerekiyorsa yeni bir `app/src/home.rs`. `agent/` crate'ine, `vision.rs`'e, `planner.rs` mantığına, `live_training.rs`'in yazma mantığına ve bellek yazma/undo koduna dokunma. Davranış aynı kalır, yalnızca sunum değişir.
2. **Yeni yazma yolu yok:** FM'ye yazan her eylem mevcut fonksiyonları çağırmaya devam eder.
3. **Çeviriler:** Tüm metinler `t!("nl", "en", "tr")` ile yazılır. FR/IT/ES için İngilizce metni anahtar olarak `i18n_fr.rs`, `i18n_it.rs` ve `i18n_es.rs` dosyalarına ekle.
4. **Bağımlılıklar:** Yeni bağımlılık eklemeden önce sor.
5. **Git:** Push, tag ve release yok. Her faz sonunda yerel commit at, mesajı `ui: faz N – <konu>` biçiminde olsun.
6. **Her faz sonunda kontrol:** `cargo build -p fm27-trainer`, `cargo test -p fm27-trainer` ve `cargo clippy -p fm27-trainer` çalıştırılır. Hepsi geçmeli ve yeni uyarı eklenmemeli.
7. **Ekran görüntüsü:** `FM27_SNAPSHOT` ile alınır. Bu değişken açıkken uygulama FM'ye yazmaz.
   `FM27_LANG=tr FM27_WINDOW=1320x880 FM27_PAGE=<sayfa> FM27_SNAPSHOT=shots/faz<N>-<sayfa>.png`
   Yeni sayfalar için `FM27_PAGE` değerlerini genişlet (`home`, `players`, `calendar`).
8. **Faz sonunda dur:** 5–8 maddelik bir özet yaz, ekran görüntülerinin yollarını ver ve onay bekle.

---

## FAZ 1: Hızlı kazanımlar

a) **Tek kaynaklı skor bantları.** `model.rs`'e `enum Band { VeryLow, Low, Fair, Good, Excellent }` ve `band(score)` ekle (eşikler: <25, <45, <60, <80, ≥80). `level_name()` ve `level_color()` bunu kullansın. Gradyanı kaldır, her bandın tek ve sabit rengi olsun: VeryLow/Low = DANGER tonu, Fair = WARNING, Good = açık yeşil, Excellent = SUCCESS. Bir test kırılırsa testi değiştirmeden önce bildir.

b) **Okunurluk.** `theme.rs`'te `TEXT_FAINT`'i bir ton aç (≈ 120,127,145). `caps()` etiketlerini 12–13 px'e çıkar. Büyük harf yalnızca kart başlıklarında kalsın.

c) **Tek güncelleme butonu.** Genel Bakış'taki üç buton yerine tek bir "Verileri güncelle" butonu koy. Kaynak otomatik seçilir: önce eklenti, yoksa ekran, o da yoksa dosya. Yanına bir ▾ menüsü ekle: "Oyun eklentisinden oku", "Ekrandan oku", "Dosyadan oku" ve "Ekrandan otomatik oku" (mevcut toggle buraya taşınır).

d) **Dil seçimi.** Kenar çubuğundaki dil anahtarını kaldır. Dil Ayarlar'da, 6 dili içeren bir açılır listeden seçilsin.

e) **Tekrarlanan güvenceler.** "%100 hilesiz…" gibi parantez içi uyarıları kartlardan sil. Bu güvence Ayarlar'da ve ilk kurulumda bir kez yazsın.

## FAZ 2: Global bağlantı durumu ve boş durumlar

a) **Global şerit.** Bağlantı yoksa veya veri okunmamışsa tüm sayfaların üstünde tek bir ince şerit göster: durum noktası, kısa metin ve "Nasıl bağlanırım?" eylemi.

b) **Kurulum akışı.** Bu eylem 3 adımlı bir akış açar (`step_card` kullanılabilir): 1) FM27'yi başlat, 2) Kariyerini yükle, 3) Taktiksel Verimlilik ekranını aç → Verileri güncelle. Tamamlanan adım ✓ ile işaretlenir; bilgi mevcut durumdan türetilir.

c) **Bireysel boş durumu.** Bağlantı yoksa Bireysel'deki üç "bağlanın" kartı yerine tek bir boş durum göster: başlık, bir satır açıklama ve tek bir buton.

d) **Kenar çubuğu.** Alttaki bağlantı kutusu şunları göstersin: durum noktası, kulüp adı, son okuma saati.

## FAZ 3: "Bu hafta" ana ekranı

`Page::Home` ekle; varsayılan sayfa bu olsun. Kodu `app/src/home.rs` dosyasına yaz.

Menü sırası (TR): **Bu hafta · Takım analizi · Takvim · Oyuncular · Taktik · Ayarlar.** Training ve Planning bu fazda ayrı kalabilir; birleştirme Faz 5'te.

Yukarıdan aşağıya düzen:

1. **Başlık satırı.** "Bu hafta · 23–29 Kas" (ay adları `i18n::month_short` ile). Altında durum satırı: ● bağlantı · Son okuma HH:MM · Sonraki maç (fikstür verisi yoksa bu parça gizlenir). Sağda "Güncelle" (ikincil) ve "Planı FM'ye uygula" (birincil) butonları. Birincil buton mevcut yazma fonksiyonunu çağırır. Başarılı olunca "Uygulandı · HH:MM" yazar; mevcut bir undo varsa yanında "Geri al" linki çıkar.

2. **4 bileşen kutucuğu.** Her kutucukta ad, skor ve `history.rs`'teki önceki ölçüme göre ▲/▼ delta olur. Ring kullanılmaz.

3. **"Öncelikler" kartı.** En düşük 3 faktör: ad, band renginde skor ve "plan bunun için ne yapıyor" açıklaması (bu faktörü hedefleyen seansların günleri, ör. "Çar · Topla diziliş"). Antrenmanla artmayan faktörlerde mevcut "antrenman dışı" öneri metni kullanılır.

4. **Kompakt 7 günlük takvim.** Faz 5'teki çip stiliyle.

5. **"Dikkat isteyen oyuncular" kartı.** En fazla 5 oyuncu: düşük kondisyon, yüksek darboğaz, 32+ yaş ve yoğun fikstür. Satır biçimi: ad · sorun · öneri. Sağ üstte "Önerileri uygula (N)" butonu mevcut fonksiyonları çağırır.

Veri yoksa her kart kendi tek satırlık boş durumunu gösterir; sayfa yine de düzgün görünmeli.

## FAZ 4: Takım analizi ve Oyuncular

a) **Takım analizi** (eski Genel Bakış). Tekrar eden bileşen adlarını kaldır. Solda 4 bileşen ve trend grafiği, sağda 17 faktör olsun. Faktörler "skora göre" ve "bileşene göre" sıralanabilsin (`segmented`). Her satırda ad, bar, skor ve delta bulunur.

b) **Oyuncular** (`individual.rs`). Darboğaz, Yoğunluk ve Ek Odak tek bir tabloda birleşir. Sütunlar: Oyuncu · Mevki · Kondisyon · Risk · Taktik puanı · Önerilen yoğunluk · Önerilen ek odak. Sütun başlığına tıklayınca sıralama yapılır. Satır başına buton olmaz; satırlar checkbox ile seçilir ve toplu işlem butonu "Seçilenlere önerileri uygula (N)" olur. Yoğunluk profili ve otomatik uygulama ayarı tablonun üstünde tek bir satırda durur.

## FAZ 5: Takvim

a) **Tek sayfa.** Antrenman ve Planlama tek bir "Takvim" sayfasında birleşir; üstte Hafta/Sezon seçimi bulunur. Oyun tarzı ve kamp modu "Plan ayarları" bölümüne taşınır.

b) **Seans çipi** (`session_chip`). Nötr zemin, solda 3 px kategori şeridi, en az 12 px yazı, ≈ 24 px yükseklik. Maç günü çipi kalın yazılır.

c) **Hafta ayarları.** Hafta başlangıcı, maç günü, önceki/sonraki gün maç bilgisi ve dönem seçimi "Hafta ayarları ▾" popup'ına taşınır. Varsayılanlar mevcut otomatik mantıktan gelir.

d) **Renk açıklaması.** Takvimin altından alınıp başlıktaki ⓘ tooltip'ine taşınır.

e) **Plan adları.** Şablon kodu yerine okunur bir açıklama gösterilir: "Dengeli hafta · Topa sahipken diziliş + pres". Kod yalnızca tooltip'te kalır.

f) **Etkinlik kaydı.** `live.rs`'te otomatik takip bir günü başarıyla geri yazdığında, bellekteki bir listeye (son 20 kayıt) şu olay eklenir: "HH:MM · FM asistanı <gün> gününü değiştirdi, geri alındı". Yazma mantığına dokunulmaz; yalnızca başarı noktasına bir log satırı eklenir. Takvim başlığında son olay görünür, tıklayınca tüm liste açılır. Geri alınan güne `icon::SHIELD_CHECK` konur.

## FAZ 6: Cila ve kontrol

- **Dil kontrolü.** Tüm sayfalar TR, EN, NL ve FR dillerinde `FM27_SNAPSHOT` ile çekilir. Taşma, kesilen metin veya üst üste binen öğe varsa düzeltilir.
- **Küçük pencere.** 1100x760 pencerede de düzen bozulmamalı; dar ekranda kartlar alt alta dizilir.
- **Temizlik.** Kullanılmayan bileşenler ve çeviri anahtarları silinir.
- **README görselleri.** `docs/images` altındaki görsellerin üzerine yazılmaz; yeni görüntüler `shots/` klasöründe kalır.
- **Son rapor.** Değişen dosyalar, fazlara göre özet, bilinen eksikler ve FM'ye bağlıyken elle test edilmesi gereken akışlar (plan uygula, öneri uygula, geri al).
