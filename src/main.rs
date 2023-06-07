use std::f64::consts::PI;
use std::fmt;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use actix_cors::Cors;

#[derive(Serialize)]
struct Complex {
    re: f64,
    im: f64,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Complex {{ re: {}, im: {} }}", self.re, self.im)
    }
}

#[derive(Serialize)]
struct FFTResponse {
    numeros_fft: Vec<Complex>,
    magnitudes: Vec<f64>,
    fases: Vec<f64>,
}

#[get("/fft")]
async fn fft_handler() -> impl Responder {
    let mut numeros = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let numeros_fft = fft(&mut numeros);
    let magnitudes = calcular_magnitudes(&numeros_fft);
    let fases = calcular_fases(&numeros_fft);
    
    let response = FFTResponse {
        numeros_fft,
        magnitudes,
        fases,
    };

    HttpResponse::Ok().json(response)
}

fn fft(numeros: &mut [i32]) -> Vec<Complex> {
    let n = numeros.len();

    if n > 1 {
        let mut par = extraer_pares(numeros);
        let mut impar = extraer_impares(numeros);

        let mut numeros: Vec<Complex> = Vec::with_capacity(n);
        for _ in 0..n {
            numeros.push(Complex { re: 0.0, im: 0.0 });
        }

        let par_fft = fft(&mut par);
        let impar_fft = fft(&mut impar);

        for m in 0..n / 2 {
            let w = Complex {
                re: (2.0 * PI * m as f64 / n as f64).cos(),
                im: -(2.0 * PI * m as f64 / n as f64).sin(),
            };

            let z = Complex {
                re: w.re * impar_fft[m].re - w.im * impar_fft[m].im,
                im: w.re * impar_fft[m].im + w.im * impar_fft[m].re,
            };

            numeros[m].re = par_fft[m].re + z.re;
            numeros[m].im = par_fft[m].im + z.im;

            numeros[m + n / 2].re = par_fft[m].re - z.re;
            numeros[m + n / 2].im = par_fft[m].im - z.im;
        }

        numeros
    } else {
        let complex_num = Complex {
            re: numeros[0] as f64,
            im: 0.0,
        };
        vec![complex_num]
    }
}

fn extraer_pares(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter()
        .enumerate()
        .filter_map(|(i, num)| if i % 2 == 0 { Some(*num) } else { None })
        .collect()
}

fn extraer_impares(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter()
        .enumerate()
        .filter_map(|(i, num)| if i % 2 != 0 { Some(*num) } else { None })
        .collect()
}

fn calcular_magnitudes(numeros: &[Complex]) -> Vec<f64> {
    numeros.iter().map(|num| (num.re.powi(2) + num.im.powi(2)).sqrt()).collect()
}

fn calcular_fases(numeros: &[Complex]) -> Vec<f64> {
    numeros.iter().map(|num| num.im.atan2(num.re)).collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method();
        App::new()
            .wrap(cors)
          
        .service(fft_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 
//url de la conexion  http://localhost:8080/fft