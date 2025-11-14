// src/system/scheduler.rs
// Planificateur pour l'indexation automatique

use chrono::{Local, Timelike};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub struct Scheduler {
    enabled: Arc<AtomicBool>,
    schedule_hour: Arc<Mutex<u32>>,    // Heure de la journée (0-23)
    schedule_minute: Arc<Mutex<u32>>,  // Minute (0-59)
    last_run: Arc<Mutex<Option<chrono::DateTime<Local>>>>,
}

impl Scheduler {
    pub fn new(hour: u32, minute: u32) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(false)),
            schedule_hour: Arc::new(Mutex::new(hour.min(23))),
            schedule_minute: Arc::new(Mutex::new(minute.min(59))),
            last_run: Arc::new(Mutex::new(None)),
        }
    }

    /// Démarrer le scheduler
    pub fn start<F>(&self, on_trigger: F)
    where
        F: Fn() + Send + 'static,
    {
        self.enabled.store(true, Ordering::Relaxed);

        let enabled = self.enabled.clone();
        let schedule_hour = self.schedule_hour.clone();
        let schedule_minute = self.schedule_minute.clone();
        let last_run = self.last_run.clone();

        thread::spawn(move || {
            while enabled.load(Ordering::Relaxed) {
                // Vérifier si c'est l'heure de lancer l'indexation
                let now = Local::now();
                let target_hour = *schedule_hour.lock().unwrap();
                let target_minute = *schedule_minute.lock().unwrap();

                let should_run = now.hour() == target_hour && now.minute() == target_minute;

                if should_run {
                    // Vérifier si on n'a pas déjà exécuté aujourd'hui
                    let mut last = last_run.lock().unwrap();
                    let already_run_today = if let Some(last_time) = *last {
                        last_time.date_naive() == now.date_naive()
                    } else {
                        false
                    };

                    if !already_run_today {
                        // Exécuter le callback
                        on_trigger();

                        // Mettre à jour le dernier lancement
                        *last = Some(now);
                    }
                }

                // Vérifier toutes les 30 secondes
                thread::sleep(Duration::from_secs(30));
            }
        });
    }

    /// Arrêter le scheduler
    pub fn stop(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    /// Vérifier si le scheduler est actif
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Changer l'horaire planifié
    pub fn set_schedule(&self, hour: u32, minute: u32) {
        *self.schedule_hour.lock().unwrap() = hour.min(23);
        *self.schedule_minute.lock().unwrap() = minute.min(59);
    }

    /// Récupérer l'horaire actuel
    pub fn get_schedule(&self) -> (u32, u32) {
        let hour = *self.schedule_hour.lock().unwrap();
        let minute = *self.schedule_minute.lock().unwrap();
        (hour, minute)
    }

    /// Récupérer le dernier lancement
    pub fn last_run(&self) -> Option<chrono::DateTime<Local>> {
        *self.last_run.lock().unwrap()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        // Par défaut: 2h00 AM
        Self::new(2, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_scheduler_new() {
        let scheduler = Scheduler::new(14, 30);
        assert_eq!(scheduler.get_schedule(), (14, 30));
        assert!(!scheduler.is_enabled());
    }

    #[test]
    fn test_scheduler_clamp_values() {
        // Heures et minutes invalides doivent être clampées
        let scheduler = Scheduler::new(25, 70);
        assert_eq!(scheduler.get_schedule(), (23, 59));
    }

    #[test]
    fn test_scheduler_set_schedule() {
        let scheduler = Scheduler::new(10, 0);
        scheduler.set_schedule(15, 45);
        assert_eq!(scheduler.get_schedule(), (15, 45));
    }

    #[test]
    fn test_scheduler_default() {
        let scheduler = Scheduler::default();
        assert_eq!(scheduler.get_schedule(), (2, 0));
    }

    #[test]
    fn test_scheduler_last_run_initially_none() {
        let scheduler = Scheduler::new(10, 0);
        assert!(scheduler.last_run().is_none());
    }

    #[test]
    fn test_scheduler_start_stop() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let scheduler = Scheduler::new(10, 0);
        scheduler.start(move || {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert!(scheduler.is_enabled());

        scheduler.stop();
        thread::sleep(Duration::from_millis(100));
        assert!(!scheduler.is_enabled());
    }
}
